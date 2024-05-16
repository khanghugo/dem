use nom::bytes::complete::take;
use nom::combinator::{fail, map};
use nom::error::context;
use nom::number::complete::le_u8;

use nom::{
    multi::count,
    number::complete::{le_i16, le_i32, le_i8, le_u16, le_u32},
    sequence::tuple,
};

use crate::nom_helper::{null_string, Result};

use crate::bit::{BitReader, BitSliceCast};
use crate::byte_writer::ByteWriter;

use crate::types::{NetMessage, UserMessage};
use crate::Aux;
use crate::{
    bit::BitWriter,
    delta::{parse_delta, write_delta},
    types::{
        BitVec, ByteVec, ClientDataWeaponData, Delta, DeltaDecoder, DeltaDecoderS,
        DeltaDecoderTable, EngineMessage, EntityStateDelta, EventS, OriginCoord, SvcAddAngle,
        SvcCdTrack, SvcCenterPrint, SvcClientData, SvcCrosshairAngle, SvcCustomization,
        SvcCutscene, SvcDecalName, SvcDeltaDescription, SvcDeltaPacketEntities, SvcDirector,
        SvcDisconnect, SvcEvent, SvcEventReliable, SvcFileTxferFailed, SvcFinale, SvcHltv,
        SvcLightStyle, SvcNewMoveVars, SvcNewUserMsg, SvcPacketEntities, SvcParticle, SvcPings,
        SvcPrint, SvcResourceList, SvcResourceLocation, SvcResourceRequest, SvcRestore,
        SvcRoomType, SvcSendCvarValue, SvcSendCvarValue2, SvcSendExtraInfo, SvcServerInfo,
        SvcSetAngle, SvcSetPause, SvcSetView, SvcSignOnNum, SvcSound, SvcSoundFade,
        SvcSpawnBaseline, SvcSpawnStatic, SvcSpawnStaticSound, SvcStopSound, SvcStuffText,
        SvcTempEntity, SvcTime, SvcTimeScale, SvcUpdateUserInfo, SvcVersion, SvcVoiceData,
        SvcVoiceInit, SvcWeaponAnim,
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
mod disconnnect;
mod event;
mod event_reliable;
mod new_user_msg;
mod packet_entities;
mod resource_list;
mod server_info;
mod sound;
mod spawn_baseline;
mod temp_entity;

trait Doer {
    fn id(&self) -> u8;
    fn parse(i: &[u8], aux: Aux) -> Result<Self> where Self: Sized;
    fn write(&self, aux: Aux) -> ByteVec;
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

impl NetMessage {
    pub fn parse(i: &[u8], aux: Aux) -> Result<NetMessage> {
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

    pub fn write(&self, aux: Aux) -> ByteVec {
        match self {
            NetMessage::UserMessage(what) => what.write(aux),
            NetMessage::EngineMessage(what) => what.write(aux),
        }
    }
}

impl UserMessage {
    fn parse(i: &[u8], id: u8, mut aux: Aux) -> Result<UserMessage> {
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
                    custom_message.unwrap().name.clone()
                } else {
                    b"\0".to_vec()
                },
                data: data.to_vec(),
            },
        ))
    }

    fn write(&self, aux: Aux) -> ByteVec {
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
    fn parse(i: &[u8], type_: u8, aux: Aux) -> Result<EngineMessage> {
        let (i, res) = match type_ {
            0 => (i, EngineMessage::SvcBad),
            1 => (i, EngineMessage::SvcNop),
            2 => wrap!(SvcDisconnect, i, aux),
            3 => wrap!(SvcEvent, i, aux),
            6 => wrap!(SvcSound, i, aux),
            11 => wrap!(SvcServerInfo, i, aux), // mutate max_client
            14 => wrap!(SvcDeltaDescription, i, aux), // mutate delta_decoders
            15 => wrap!(SvcClientData, i, aux),
            21 => wrap!(SvcEventReliable, i, aux),
            22 => wrap!(SvcSpawnBaseline, i, aux),
            26 => wrap!(SvcCenterPrint, i, aux),
            32 => wrap!(SvcCdTrack, i, aux),
            34 => wrap!(SvcCutscene, i, aux),
            36 => wrap!(SvcDecalName, i, aux),
            38 => wrap!(SvcAddAngle, i, aux),
            39 => wrap!(SvcNewUserMsg, i, aux), // mutate custom_messages
            40 => wrap!(SvcPacketEntities, i, aux),
            41 => wrap!(SvcDeltaDescription, i, aux),
            43 => wrap!(SvcResourceList, i, aux),
            46 => wrap!(SvcCustomization, i, aux),
            47 => wrap!(SvcCrosshairAngle, i, aux),
            _ => context("Bad engine message number", fail)(i)?,
        };

        Ok((i, res))
    }

    fn write(&self, aux: Aux) -> ByteVec {
        match self {
            EngineMessage::SvcBad => vec![self.id()],
            EngineMessage::SvcNop => vec![self.id()],
            EngineMessage::SvcDisconnect(what) => what.write(aux),
            EngineMessage::SvcEvent(what) => what.write(aux),
            EngineMessage::SvcVersion(_) => todo!(),
            EngineMessage::SvcSetView(_) => todo!(),
            EngineMessage::SvcSound(what) => what.write(aux),
            EngineMessage::SvcTime(_) => todo!(),
            EngineMessage::SvcPrint(_) => todo!(),
            EngineMessage::SvcStuffText(_) => todo!(),
            EngineMessage::SvcSetAngle(_) => todo!(),
            EngineMessage::SvcServerInfo(what) => what.write(aux),
            EngineMessage::SvcLightStyle(_) => todo!(),
            EngineMessage::SvcUpdateUserInfo(_) => todo!(),
            EngineMessage::SvcDeltaDescription(_) => todo!(),
            EngineMessage::SvcClientData(_) => todo!(),
            EngineMessage::SvcStopSound(_) => todo!(),
            EngineMessage::SvcPings(_) => todo!(),
            EngineMessage::SvcParticle(_) => todo!(),
            EngineMessage::SvcDamage => todo!(),
            EngineMessage::SvcSpawnStatic(_) => todo!(),
            EngineMessage::SvcEventReliable(_) => todo!(),
            EngineMessage::SvcSpawnBaseline(_) => todo!(),
            EngineMessage::SvcTempEntity(what) => what.write(aux),
            EngineMessage::SvcSetPause(_) => todo!(),
            EngineMessage::SvcSignOnNum(_) => todo!(),
            EngineMessage::SvcCenterPrint(_) => todo!(),
            EngineMessage::SvcKilledMonster => todo!(),
            EngineMessage::SvcFoundSecret => todo!(),
            EngineMessage::SvcSpawnStaticSound(_) => todo!(),
            EngineMessage::SvcIntermission => todo!(),
            EngineMessage::SvcFinale(_) => todo!(),
            EngineMessage::SvcCdTrack(what) => what.write(aux),
            EngineMessage::SvcRestore(_) => todo!(),
            EngineMessage::SvcCutscene(what) => what.write(aux),
            EngineMessage::SvcWeaponAnim(_) => todo!(),
            EngineMessage::SvcDecalName(what) => what.write(aux),
            EngineMessage::SvcRoomType(_) => todo!(),
            EngineMessage::SvcAddAngle(_) => todo!(),
            EngineMessage::SvcNewUserMsg(_) => todo!(),
            EngineMessage::SvcPacketEntities(_) => todo!(),
            EngineMessage::SvcDeltaPacketEntities(_) => todo!(),
            EngineMessage::SvcChoke => todo!(),
            EngineMessage::SvcResourceList(what) => what.write(aux),
            EngineMessage::SvcNewMovevars(_) => todo!(),
            EngineMessage::SvcResourceRequest(_) => todo!(),
            EngineMessage::SvcCustomization(_) => todo!(),
            EngineMessage::SvcCrosshairAngle(_) => todo!(),
            EngineMessage::SvcSoundFade(_) => todo!(),
            EngineMessage::SvcFileTxferFailed(_) => todo!(),
            EngineMessage::SvcHltv(_) => todo!(),
            EngineMessage::SvcDirector(_) => todo!(),
            EngineMessage::SvcVoiceInit(_) => todo!(),
            EngineMessage::SvcVoiceData(_) => todo!(),
            EngineMessage::SvcSendExtraInfo(_) => todo!(),
            EngineMessage::SvcTimeScale(_) => todo!(),
            EngineMessage::SvcResourceLocation(_) => todo!(),
            EngineMessage::SvcSendCvarValue(_) => todo!(),
            EngineMessage::SvcSendCvarValue2(_) => todo!(),
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
