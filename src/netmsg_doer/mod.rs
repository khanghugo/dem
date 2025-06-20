use nom::bytes::complete::take;
use nom::combinator::{fail, map};
use nom::error::context;
use nom::number::complete::le_u8;

use nom::{
    multi::count,
    number::complete::{le_f32, le_i16, le_i32, le_i8, le_u16, le_u32},
    sequence::tuple,
};

use crate::nom_helper::{null_string, Result};

use crate::bit::{BitReader, BitSliceCast};
use crate::byte_writer::ByteWriter;

use crate::types::{AuxRefCell, NetMessage, UserMessage};
use crate::{
    bit::BitWriter,
    delta::{parse_delta, write_delta},
    types::{
        BitVec, ByteVec, EngineMessage, SvcAddAngle, SvcCdTrack, SvcCenterPrint, SvcClientData,
        SvcCrosshairAngle, SvcCustomization, SvcCutscene, SvcDecalName, SvcDeltaDescription,
        SvcDeltaPacketEntities, SvcDirector, SvcDisconnect, SvcEvent, SvcEventReliable,
        SvcFileTxferFailed, SvcFinale, SvcHltv, SvcLightStyle, SvcNewMovevars, SvcNewUserMsg,
        SvcPacketEntities, SvcParticle, SvcPings, SvcPrint, SvcResourceList, SvcResourceLocation,
        SvcResourceRequest, SvcRestore, SvcRoomType, SvcSendCvarValue, SvcSendCvarValue2,
        SvcSendExtraInfo, SvcServerInfo, SvcSetAngle, SvcSetPause, SvcSetView, SvcSignOnNum,
        SvcSound, SvcSoundFade, SvcSpawnBaseline, SvcSpawnStatic, SvcSpawnStaticSound,
        SvcStopSound, SvcStuffText, SvcTempEntity, SvcTime, SvcTimeScale, SvcUpdateUserInfo,
        SvcVersion, SvcVoiceData, SvcVoiceInit, SvcWeaponAnim,
    },
};

// main stuffs
mod add_angle;
mod cd_track;
mod center_print;
mod client_data;
mod crosshair_angle;
mod customization;
mod cutscene;
mod decal_name;
mod delta_description;
mod delta_packet_entities;
mod director;
mod disconnect;
mod event;
mod event_reliable;
mod file_txfer_failed;
mod finale;
mod hltv;
mod light_style;
mod new_movevars;
mod new_user_msg;
mod packet_entities;
mod particle;
mod pings;
mod print;
mod resource_list;
mod resource_location;
mod resource_request;
mod restore;
mod room_type;
mod send_cvar_value;
mod send_cvar_value_2;
mod send_extra_info;
mod server_info;
mod set_angle;
mod set_pause;
mod set_view;
mod sign_on_num;
mod sound;
mod sound_fade;
mod spawn_baseline;
mod spawn_static;
mod spawn_static_sound;
mod stop_sound;
mod stuff_text;
mod temp_entity;
mod time;
mod time_scale;
mod update_user_info;
mod version;
mod voice_data;
mod voice_init;
mod weapon_anim;

pub trait Doer {
    fn id(&self) -> u8;
    fn parse(i: &[u8], aux: AuxRefCell) -> Result<Self>
    where
        Self: Sized;
    fn write(&self, aux: AuxRefCell) -> ByteVec;
}

macro_rules! wrap {
    ($svc:ident, $parser:ident, $input:ident, $aux:ident) => {{
        let ($input, res) = $parser::parse($input, $aux)?;
        ($input, EngineMessage::$svc(res))
    }};

    // This one means the struct name has to be the same as enum name
    ($svc:ident, $input:ident, $aux:ident) => {{
        let ($input, res) = $svc::parse($input, $aux)?;
        ($input, EngineMessage::$svc(res))
    }};
}

macro_rules! wrap_box {
    ($svc:ident, $parser:ident, $input:ident, $aux:ident) => {{
        let ($input, res) = $parser::parse($input, $aux)?;
        ($input, EngineMessage::$svc(Box::new(res)))
    }};

    // This one means the struct name has to be the same as enum name
    ($svc:ident, $input:ident, $aux:ident) => {{
        let ($input, res) = $svc::parse($input, $aux)?;
        ($input, EngineMessage::$svc(Box::new(res)))
    }};
}

impl NetMessage {
    pub fn parse(i: &[u8], aux: AuxRefCell) -> Result<NetMessage> {
        let (i, type_) = le_u8(i)?;

        match type_ {
            0..=63 => {
                let (i, res) = EngineMessage::parse(i, type_, aux)?;
                Ok((i, NetMessage::EngineMessage(Box::new(res))))
            }
            _ => {
                let (i, res) = UserMessage::parse(i, type_, aux)?;
                Ok((i, NetMessage::UserMessage(res)))
            }
        }
    }

    pub fn write(&self, aux: AuxRefCell) -> ByteVec {
        match self {
            NetMessage::UserMessage(what) => what.write(aux),
            NetMessage::EngineMessage(what) => what.write(aux),
        }
    }
}

impl UserMessage {
    fn parse(i: &[u8], id: u8, aux: AuxRefCell) -> Result<UserMessage> {
        let aux = aux.borrow();

        let custom_message = aux.custom_messages.get(&id);

        let is_set = custom_message.is_some();
        let is_size = custom_message.is_some() && custom_message.unwrap().size > -1; // equivalent to -1

        let (i, data) = if is_size {
            take(custom_message.unwrap().size as usize)(i)?
        } else {
            let (i, length) = le_u8(i)?;
            take(length as usize)(i)?
        };

        Ok((
            i,
            UserMessage {
                id,
                name: if is_set {
                    custom_message.unwrap().name.clone().into()
                } else {
                    vec![0].into()
                },
                data: data.to_vec(),
            },
        ))
    }

    fn write(&self, aux: AuxRefCell) -> ByteVec {
        let aux = aux.borrow();

        let mut writer = ByteWriter::new();

        writer.append_u8(self.id);

        if let Some(message) = aux.custom_messages.get(&self.id) {
            if message.size == -1 {
                writer.append_u8(self.data.len() as u8);
            }
        }

        writer.append_u8_slice(&self.data);

        writer.data
    }
}

impl EngineMessage {
    fn parse(i: &[u8], type_: u8, aux: AuxRefCell) -> Result<EngineMessage> {
        let (i, res) = match type_ {
            0 => (i, EngineMessage::SvcBad),
            1 => (i, EngineMessage::SvcNop),
            2 => wrap!(SvcDisconnect, i, aux),
            3 => wrap!(SvcEvent, i, aux),
            4 => wrap!(SvcVersion, i, aux),
            5 => wrap!(SvcSetView, i, aux),
            6 => wrap_box!(SvcSound, i, aux),
            7 => wrap!(SvcTime, i, aux),
            8 => wrap!(SvcPrint, i, aux),
            9 => wrap!(SvcStuffText, i, aux),
            10 => wrap!(SvcSetAngle, i, aux),
            11 => wrap!(SvcServerInfo, i, aux), // mutate max_client
            12 => wrap!(SvcLightStyle, i, aux),
            13 => wrap!(SvcUpdateUserInfo, i, aux),
            14 => wrap!(SvcDeltaDescription, i, aux), // mutate delta_decoders
            15 => wrap!(SvcClientData, i, aux),
            16 => wrap!(SvcStopSound, i, aux),
            17 => wrap!(SvcPings, i, aux),
            18 => wrap!(SvcParticle, i, aux),
            19 => (i, EngineMessage::SvcDamage),
            20 => wrap!(SvcSpawnStatic, i, aux),
            21 => wrap!(SvcEventReliable, i, aux),
            22 => wrap!(SvcSpawnBaseline, i, aux),
            23 => wrap!(SvcTempEntity, i, aux),
            24 => wrap!(SvcSetPause, i, aux),
            25 => wrap!(SvcSignOnNum, i, aux),
            26 => wrap!(SvcCenterPrint, i, aux),
            27 => (i, EngineMessage::SvcKilledMonster),
            28 => (i, EngineMessage::SvcFoundSecret),
            29 => wrap!(SvcSpawnStaticSound, i, aux),
            30 => (i, EngineMessage::SvcIntermission),
            31 => wrap!(SvcFinale, i, aux),
            32 => wrap!(SvcCdTrack, i, aux),
            33 => wrap!(SvcRestore, i, aux),
            34 => wrap!(SvcCutscene, i, aux),
            35 => wrap!(SvcWeaponAnim, i, aux),
            36 => wrap!(SvcDecalName, i, aux),
            37 => wrap!(SvcRoomType, i, aux),
            38 => wrap!(SvcAddAngle, i, aux),
            39 => wrap!(SvcNewUserMsg, i, aux), // mutate custom_messages
            40 => wrap!(SvcPacketEntities, i, aux),
            41 => wrap!(SvcDeltaPacketEntities, i, aux),
            42 => (i, EngineMessage::SvcChoke),
            43 => wrap!(SvcResourceList, i, aux),
            44 => wrap!(SvcNewMovevars, i, aux),
            45 => wrap!(SvcResourceRequest, i, aux),
            46 => wrap!(SvcCustomization, i, aux),
            47 => wrap!(SvcCrosshairAngle, i, aux),
            48 => wrap!(SvcSoundFade, i, aux),
            49 => wrap!(SvcFileTxferFailed, i, aux),
            50 => wrap!(SvcHltv, i, aux),
            51 => wrap!(SvcDirector, i, aux),
            52 => wrap!(SvcVoiceInit, i, aux),
            53 => wrap!(SvcVoiceData, i, aux),
            54 => wrap!(SvcSendExtraInfo, i, aux),
            55 => wrap!(SvcTimeScale, i, aux),
            56 => wrap!(SvcResourceLocation, i, aux),
            57 => wrap!(SvcSendCvarValue, i, aux),
            58 => wrap!(SvcSendCvarValue2, i, aux),
            59..=63 => (i, EngineMessage::SvcBad),
            _ => context("Bad engine message number", fail)(i)?,
        };

        Ok((i, res))
    }

    fn write(&self, aux: AuxRefCell) -> ByteVec {
        match self {
            EngineMessage::SvcBad => vec![self.id()],
            EngineMessage::SvcNop => vec![self.id()],
            EngineMessage::SvcDisconnect(what) => what.write(aux),
            EngineMessage::SvcEvent(what) => what.write(aux),
            EngineMessage::SvcVersion(what) => what.write(aux),
            EngineMessage::SvcSetView(what) => what.write(aux),
            EngineMessage::SvcSound(what) => what.write(aux),
            EngineMessage::SvcTime(what) => what.write(aux),
            EngineMessage::SvcPrint(what) => what.write(aux),
            EngineMessage::SvcStuffText(what) => what.write(aux),
            EngineMessage::SvcSetAngle(what) => what.write(aux),
            EngineMessage::SvcServerInfo(what) => what.write(aux),
            EngineMessage::SvcLightStyle(what) => what.write(aux),
            EngineMessage::SvcUpdateUserInfo(what) => what.write(aux),
            EngineMessage::SvcDeltaDescription(what) => what.write(aux),
            EngineMessage::SvcClientData(what) => what.write(aux),
            EngineMessage::SvcStopSound(what) => what.write(aux),
            EngineMessage::SvcPings(what) => what.write(aux),
            EngineMessage::SvcParticle(what) => what.write(aux),
            EngineMessage::SvcDamage => vec![self.id()],
            EngineMessage::SvcSpawnStatic(what) => what.write(aux),
            EngineMessage::SvcEventReliable(what) => what.write(aux),
            EngineMessage::SvcSpawnBaseline(what) => what.write(aux),
            EngineMessage::SvcTempEntity(what) => what.write(aux),
            EngineMessage::SvcSetPause(what) => what.write(aux),
            EngineMessage::SvcSignOnNum(what) => what.write(aux),
            EngineMessage::SvcCenterPrint(what) => what.write(aux),
            EngineMessage::SvcKilledMonster => vec![self.id()],
            EngineMessage::SvcFoundSecret => vec![self.id()],
            EngineMessage::SvcSpawnStaticSound(what) => what.write(aux),
            EngineMessage::SvcIntermission => vec![self.id()],
            EngineMessage::SvcFinale(what) => what.write(aux),
            EngineMessage::SvcCdTrack(what) => what.write(aux),
            EngineMessage::SvcRestore(what) => what.write(aux),
            EngineMessage::SvcCutscene(what) => what.write(aux),
            EngineMessage::SvcWeaponAnim(what) => what.write(aux),
            EngineMessage::SvcDecalName(what) => what.write(aux),
            EngineMessage::SvcRoomType(what) => what.write(aux),
            EngineMessage::SvcAddAngle(what) => what.write(aux),
            EngineMessage::SvcNewUserMsg(what) => what.write(aux),
            EngineMessage::SvcPacketEntities(what) => what.write(aux),
            EngineMessage::SvcDeltaPacketEntities(what) => what.write(aux),
            EngineMessage::SvcChoke => vec![self.id()],
            EngineMessage::SvcResourceList(what) => what.write(aux),
            EngineMessage::SvcNewMovevars(what) => what.write(aux),
            EngineMessage::SvcResourceRequest(what) => what.write(aux),
            EngineMessage::SvcCustomization(what) => what.write(aux),
            EngineMessage::SvcCrosshairAngle(what) => what.write(aux),
            EngineMessage::SvcSoundFade(what) => what.write(aux),
            EngineMessage::SvcFileTxferFailed(what) => what.write(aux),
            EngineMessage::SvcHltv(what) => what.write(aux),
            EngineMessage::SvcDirector(what) => what.write(aux),
            EngineMessage::SvcVoiceInit(what) => what.write(aux),
            EngineMessage::SvcVoiceData(what) => what.write(aux),
            EngineMessage::SvcSendExtraInfo(what) => what.write(aux),
            EngineMessage::SvcTimeScale(what) => what.write(aux),
            EngineMessage::SvcResourceLocation(what) => what.write(aux),
            EngineMessage::SvcSendCvarValue(what) => what.write(aux),
            EngineMessage::SvcSendCvarValue2(what) => what.write(aux),
        }
    }

    // repeat because of code being fragmented
    // for good purposes tho
    fn id(&self) -> u8 {
        match self {
            EngineMessage::SvcBad => 0,
            EngineMessage::SvcNop => 1,
            EngineMessage::SvcDisconnect(_) => 2,
            EngineMessage::SvcEvent(_) => 3,
            EngineMessage::SvcVersion(_) => 4,
            EngineMessage::SvcSetView(_) => 5,
            EngineMessage::SvcSound(_) => 6,
            EngineMessage::SvcTime(_) => 7,
            EngineMessage::SvcPrint(_) => 8,
            EngineMessage::SvcStuffText(_) => 9,
            EngineMessage::SvcSetAngle(_) => 10,
            EngineMessage::SvcServerInfo(_) => 11,
            EngineMessage::SvcLightStyle(_) => 12,
            EngineMessage::SvcUpdateUserInfo(_) => 13,
            EngineMessage::SvcDeltaDescription(_) => 14,
            EngineMessage::SvcClientData(_) => 15,
            EngineMessage::SvcStopSound(_) => 16,
            EngineMessage::SvcPings(_) => 17,
            EngineMessage::SvcParticle(_) => 18,
            EngineMessage::SvcDamage => 19,
            EngineMessage::SvcSpawnStatic(_) => 20,
            EngineMessage::SvcEventReliable(_) => 21,
            EngineMessage::SvcSpawnBaseline(_) => 22,
            EngineMessage::SvcTempEntity(_) => 23,
            EngineMessage::SvcSetPause(_) => 24,
            EngineMessage::SvcSignOnNum(_) => 25,
            EngineMessage::SvcCenterPrint(_) => 26,
            EngineMessage::SvcKilledMonster => 27,
            EngineMessage::SvcFoundSecret => 28,
            EngineMessage::SvcSpawnStaticSound(_) => 29,
            EngineMessage::SvcIntermission => 30,
            EngineMessage::SvcFinale(_) => 31,
            EngineMessage::SvcCdTrack(_) => 32,
            EngineMessage::SvcRestore(_) => 33,
            EngineMessage::SvcCutscene(_) => 34,
            EngineMessage::SvcWeaponAnim(_) => 35,
            EngineMessage::SvcDecalName(_) => 36,
            EngineMessage::SvcRoomType(_) => 37,
            EngineMessage::SvcAddAngle(_) => 38,
            EngineMessage::SvcNewUserMsg(_) => 39,
            EngineMessage::SvcPacketEntities(_) => 40,
            EngineMessage::SvcDeltaPacketEntities(_) => 41,
            EngineMessage::SvcChoke => 42,
            EngineMessage::SvcResourceList(_) => 43,
            EngineMessage::SvcNewMovevars(_) => 44,
            EngineMessage::SvcResourceRequest(_) => 45,
            EngineMessage::SvcCustomization(_) => 46,
            EngineMessage::SvcCrosshairAngle(_) => 47,
            EngineMessage::SvcSoundFade(_) => 48,
            EngineMessage::SvcFileTxferFailed(_) => 49,
            EngineMessage::SvcHltv(_) => 50,
            EngineMessage::SvcDirector(_) => 51,
            EngineMessage::SvcVoiceInit(_) => 52,
            EngineMessage::SvcVoiceData(_) => 53,
            EngineMessage::SvcSendExtraInfo(_) => 54,
            EngineMessage::SvcTimeScale(_) => 55,
            EngineMessage::SvcResourceLocation(_) => 56,
            EngineMessage::SvcSendCvarValue(_) => 57,
            EngineMessage::SvcSendCvarValue2(_) => 58,
        }
    }
}
