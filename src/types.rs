use std::collections::HashMap;

use bitvec::{order::Lsb0, vec::BitVec as _BitVec};

// use super::*;

// Primitive
// pub type BitVec = _BitVec<u8>;
pub type BitVec = _BitVec<u8, Lsb0>;
pub type ByteVec = Vec<u8>;

// Delta
pub type Delta = HashMap<String, ByteVec>;
pub type DeltaDecoder = Vec<DeltaDecoderS>;
pub type DeltaDecoderTable = HashMap<String, DeltaDecoder>;

pub struct DeltaDecoderS {
    pub name: ByteVec,
    pub bits: u32,
    pub divisor: f32,
    pub flags: u32,
}

#[repr(u32)]
pub enum DeltaType {
    Byte = 1,
    Short = 1 << 1,
    Float = 1 << 2,
    Integer = 1 << 3,
    Angle = 1 << 4,
    TimeWindow8 = 1 << 5,
    TimeWindowBig = 1 << 6,
    String = 1 << 7,
    Signed = 1 << 31,
}

// Main
pub enum NetMessage {
    UserMessage(UserMessage),
    EngineMessage(EngineMessage),
}

pub struct UserMessage {
    pub id: u8,
    // [bool; 16]
    pub name: ByteVec,
    pub data: ByteVec,
}

// Messages
#[repr(u8)]
pub enum EngineMessage {
    SvcBad = 0,
    SvcNop = 1,
    SvcDisconnect(SvcDisconnect) = 2,
    SvcEvent(SvcEvent) = 3,
    SvcVersion(SvcVersion) = 4,
    SvcSetView(SvcSetView) = 5,
    SvcSound(SvcSound) = 6,
    SvcTime(SvcTime) = 7,
    SvcPrint(SvcPrint) = 8,
    SvcStuffText(SvcStuffText) = 9,
    SvcSetAngle(SvcSetAngle) = 10,
    SvcServerInfo(SvcServerInfo) = 11,
    SvcLightStyle(SvcLightStyle) = 12,
    SvcUpdateUserInfo(SvcUpdateUserInfo) = 13,
    SvcDeltaDescription(SvcDeltaDescription) = 14,
    SvcClientData(SvcClientData) = 15,
    SvcStopSound(SvcStopSound) = 16,
    SvcPings(SvcPings) = 17,
    SvcParticle(SvcParticle) = 18,
    SvcDamage = 19,
    SvcSpawnStatic(SvcSpawnStatic) = 20,
    SvcEventReliable(SvcEventReliable) = 21,
    SvcSpawnBaseline(SvcSpawnBaseline) = 22,
    SvcTempEntity(SvcTempEntity) = 23,
    SvcSetPause(SvcSetPause) = 24,
    SvcSignOnNum(SvcSignOnNum) = 25,
    SvcCenterPrint(SvcCenterPrint) = 26,
    SvcKilledMonster = 27,
    SvcFoundSecret = 28,
    SvcSpawnStaticSound(SvcSpawnStaticSound) = 29,
    SvcIntermission = 30,
    SvcFinale(SvcFinale) = 31,
    SvcCdTrack(SvcCdTrack) = 32,
    SvcRestore(SvcRestore) = 33,
    SvcCutscene(SvcCutscene) = 34,
    SvcWeaponAnim(SvcWeaponAnim) = 35,
    SvcDecalName(SvcDecalName) = 36,
    SvcRoomType(SvcRoomType) = 37,
    SvcAddAngle(SvcAddAngle) = 38,
    SvcNewUserMsg(SvcNewUserMsg) = 39,
    SvcPacketEntities(SvcPacketEntities) = 40,
    SvcDeltaPacketEntities(SvcDeltaPacketEntities) = 41,
    SvcChoke = 42,
    SvcResourceList(SvcResourceList) = 43,
    SvcNewMovevars(SvcNewMoveVars) = 44,
    SvcResourceRequest(SvcResourceRequest) = 45,
    SvcCustomization(SvcCustomization) = 46,
    SvcCrosshairAngle(SvcCrosshairAngle) = 47,
    SvcSoundFade(SvcSoundFade) = 48,
    SvcFileTxferFailed(SvcFileTxferFailed) = 49,
    SvcHltv(SvcHltv) = 50,
    SvcDirector(SvcDirector) = 51,
    SvcVoiceInit(SvcVoiceInit) = 52,
    SvcVoiceData(SvcVoiceData) = 53,
    SvcSendExtraInfo(SvcSendExtraInfo) = 54,
    SvcTimeScale(SvcTimeScale) = 55,
    SvcResourceLocation(SvcResourceLocation) = 56,
    SvcSendCvarValue(SvcSendCvarValue) = 57,
    SvcSendCvarValue2(SvcSendCvarValue2) = 58,
}

/// SVC_BAD 0
//

/// SVC_NOP 1
//

/// SVC_DISCONNECT 2
pub struct SvcDisconnect {
    pub reason: ByteVec,
}

/// SVC_EVENT 3
pub struct SvcEvent {
    // [bool; 5]
    pub event_count: BitVec,
    pub events: Vec<EventS>,
}

pub struct EventS {
    // [bool; 10]
    pub event_index: BitVec,
    pub has_packet_index: bool,
    // [bool; 11]
    pub packet_index: Option<BitVec>,
    pub has_delta: Option<bool>,
    pub delta: Option<Delta>,
    pub has_fire_time: bool,
    // [bool; 16]
    pub fire_time: Option<BitVec>,
}

/// SVC_VERSION 4
pub struct SvcVersion {
    pub protocol_version: u32,
}

/// SVC_SETVIEW 5
pub struct SvcSetView {
    pub entity_index: i16,
}

/// SVC_SOUND 6
pub struct SvcSound {
    // [bool; 9]
    pub flags: BitVec,
    pub volume: Option<BitVec>,
    pub attenuation: Option<BitVec>,
    // [bool; 3]
    pub channel: BitVec,
    // [bool; 11]
    pub entity_index: BitVec,
    pub sound_index_long: Option<BitVec>,
    pub sound_index_short: Option<BitVec>,
    pub has_x: bool,
    pub has_y: bool,
    pub has_z: bool,
    pub origin_x: Option<OriginCoord>,
    pub origin_y: Option<OriginCoord>,
    pub origin_z: Option<OriginCoord>,
    pub pitch: BitVec,
}

pub struct OriginCoord {
    pub int_flag: bool,
    pub fraction_flag: bool,
    pub is_negative: Option<bool>,
    // [bool; 12]
    pub int_value: Option<BitVec>,
    // [bool; 3]
    pub fraction_value: Option<BitVec>,
    // There is no unknow, Xd
    // [bool; 2]
    // pub unknown: BitVec,
}

/// SVC_TIME 7
pub struct SvcTime {
    pub time: f32,
}

/// SVC_PRINT 8
pub struct SvcPrint {
    pub message: ByteVec,
}

/// SVC_STUFFTEXT 9
pub struct SvcStuffText {
    pub command: ByteVec,
}

/// SVC_SETANGLE 10
pub struct SvcSetAngle {
    pub pitch: i16,
    pub yaw: i16,
    pub roll: i16,
}

/// SVC_SERVERINFO 11
pub struct SvcServerInfo {
    pub protocol: i32,
    pub spawn_count: i32,
    pub map_checksum: i32,
    // [u8; 16]
    pub client_dll_hash: ByteVec,
    pub max_players: u8,
    pub player_index: u8,
    pub is_deathmatch: u8,
    pub game_dir: ByteVec,
    pub hostname: ByteVec,
    pub map_file_name: ByteVec,
    pub map_cycle: ByteVec,
    pub unknown: u8,
}

/// SVC_LIGHTSTYLE 12
pub struct SvcLightStyle {
    pub index: u8,
    pub light_info: ByteVec,
}

/// SVC_UPDATEUSERINFO 13
pub struct SvcUpdateUserInfo {
    pub index: u8,
    pub id: u32,
    pub user_info: ByteVec,
    // [u8; 16]
    pub cd_key_hash: ByteVec,
}

/// SVC_DELTADESCRIPTION 14
pub struct SvcDeltaDescription {
    pub name: ByteVec,
    pub total_fields: u16,
    pub fields: DeltaDecoder,
    pub clone: ByteVec,
}

/// SVC_CLIENTDATA 15
pub struct SvcClientData {
    pub has_delta_update_mask: bool,
    // [bool; 8]
    pub delta_update_mask: Option<BitVec>,
    pub client_data: Delta,
    pub weapon_data: Option<Vec<ClientDataWeaponData>>,
}

pub struct ClientDataWeaponData {
    // [bool; 6]
    pub weapon_index: BitVec,
    pub weapon_data: Delta,
}

/// SVC_STOPSOUND 16
pub struct SvcStopSound {
    pub entity_index: i16,
}

/// SVC_PINGS 17
pub struct SvcPings {
    pub pings: Vec<PingS>,
}

pub struct PingS {
    pub has_ping_data: bool,
    pub player_id: Option<u8>,
    pub ping: Option<u8>,
    pub loss: Option<u8>,
}

/// SVC_PARTICLE 18
pub struct SvcParticle {
    // Vec3
    pub origin: Vec<i16>,
    // Vec3
    pub direction: ByteVec,
    pub count: u8,
    pub color: u8,
}

/// SVC_PARTICLE 19

/// SVC_SPAWNSTATIC 20
pub struct SvcSpawnStatic {
    pub model_index: i16,
    pub sequence: i8,
    pub frame: i8,
    pub color_map: i16,
    pub skin: i8,
    pub origin_x: i16,
    pub rotation_x: i8,
    pub origin_y: i16,
    pub rotation_y: i8,
    pub origin_z: i16,
    pub rotation_z: i8,
    pub has_render_mode: i8,
    // [u8; 3]
    pub render_color: Option<ByteVec>,
}

/// SVC_EVENTRELIABLE 21
pub struct SvcEventReliable {
    // [bool; 10]
    pub event_index: BitVec,
    pub event_args: Delta,
    pub has_fire_time: bool,
    // [bool; 16]
    pub fire_time: Option<BitVec>,
}

/// SVC_SPAWNBASELINE 22
pub struct SvcSpawnBaseline {
    pub entities: Vec<EntityS>,
    // These members are not inside EntityS like cgdangelo/talent suggests.
    // [bool; 6]
    pub total_extra_data: BitVec,
    pub extra_data: Vec<Delta>,
}

pub struct EntityS {
    // Goodies
    pub entity_index: u16,
    // [bool; 11]
    pub index: BitVec,
    // [bool; 2]
    pub type_: BitVec,
    // One delta for 3 types
    pub delta: Delta,
}

/// SVC_TEMPENTITY 23
pub struct SvcTempEntity {
    pub entity_type: u8,
    pub entity: TempEntity,
}

#[repr(u8)]
pub enum TempEntity {
    // [u8; 24]
    TeBeamPoints(TeBeamPoints) = 0,
    // [u8; 20]
    TeBeamEntPoint(ByteVec) = 1,
    // [u8; 6]
    TeGunshot(ByteVec) = 2,
    // It is 11
    // [u8; 11]
    TeExplosion(ByteVec) = 3,
    // [u8; 6]
    TeTarExplosion(ByteVec) = 4,
    // [u8; 10]
    TeSmoke(ByteVec) = 5,
    // [u8; 12]
    TeTracer(ByteVec) = 6,
    // [u8; 17]
    TeLightning(ByteVec) = 7,
    // [u8; 16]
    TeBeamEnts(ByteVec) = 8,
    // [u8; 6]
    TeSparks(ByteVec) = 9,
    // [u8; 6]
    TeLavaSplash(ByteVec) = 10,
    // [u8; 6]
    TeTeleport(ByteVec) = 11,
    // [u8; 8]
    TeExplosion2(ByteVec) = 12,
    TeBspDecal(TeBspDecal) = 13,
    // [u8; 9]
    TeImplosion(ByteVec) = 14,
    // [u8; 19]
    TeSpriteTrail(ByteVec) = 15,
    // [u8; 10]
    TeSprite(ByteVec) = 16,
    // [u8; 16]
    TeBeamSprite(ByteVec) = 18,
    // [u8; 24]
    TeBeamTorus(ByteVec) = 19,
    // [u8; 24]
    TeBeamDisk(ByteVec) = 20,
    // [u8; 24]
    TeBeamCylinder(ByteVec) = 21,
    // [u8; 10]
    TeBeamFollow(ByteVec) = 22,
    // [u8; 11]
    TeGlowSprite(ByteVec) = 23,
    // [u8; 16]
    TeBeamRing(ByteVec) = 24,
    // [u8; 19]
    TeStreakSplash(ByteVec) = 25,
    // [u8; 12]
    TeDLight(ByteVec) = 27,
    // [u8; 16]
    TeELight(ByteVec) = 28,
    TeTextMessage(TeTextMessage) = 29,
    // [u8; 17]
    TeLine(ByteVec) = 30,
    // [u8; 17]
    TeBox(ByteVec) = 31,
    // [u8; 2]
    TeKillBeam(ByteVec) = 99,
    // [u8; 10]
    TeLargeFunnel(ByteVec) = 100,
    // [u8; 14]
    TeBloodStream(ByteVec) = 101,
    // [u8; 12]
    TeShowLine(ByteVec) = 102,
    // [u8; 14]
    TeBlood(ByteVec) = 103,
    // [u8; 9]
    TeDecal(ByteVec) = 104,
    // [u8; 5]
    TeFizz(ByteVec) = 105,
    // [u8; 17]
    TeModel(ByteVec) = 106,
    // [u8; 13]
    TeExplodeModel(ByteVec) = 107,
    // It is 24
    // [u8; 24]
    TeBreakModel(ByteVec) = 108,
    // [u8; 9]
    TeGunshotDecal(ByteVec) = 109,
    // [u8; 17]
    TeSpriteSpray(ByteVec) = 110,
    // [u8; 7]
    TeArmorRicochet(ByteVec) = 111,
    // [u8; 10]
    TePlayerDecal(ByteVec) = 112,
    // [u8; 10]
    TeBubbles(ByteVec) = 113,
    // [u8; 19]
    TeBubbleTrail(ByteVec) = 114,
    // [u8; 12]
    TeBloodSprite(ByteVec) = 115,
    // [u8; 7]
    TeWorldDecal(ByteVec) = 116,
    // [u8; 7]
    TeWorldDecalHigh(ByteVec) = 117,
    // [u8; 9]
    TeDecalHigh(ByteVec) = 118,
    // [u8; 16]
    TeProjectile(ByteVec) = 119,
    // [u8; 18]
    TeSpray(ByteVec) = 120,
    // [u8; 5]
    TePlayerSprites(ByteVec) = 121,
    // [u8; 10]
    TeParticleBurst(ByteVec) = 122,
    // [u8; 9]
    TeFireField(ByteVec) = 123,
    // [u8; 7]
    TePlayerAttachment(ByteVec) = 124,
    // [u8; 1]
    TeKillPlayerAttachment(ByteVec) = 125,
    // It is 18.
    // [u8; 18]
    TeMultigunShot(ByteVec) = 126,
    // [u8; 15]
    TeUserTracer(ByteVec) = 127,
}

// TE_BEAMPOINTS 0
pub struct TeBeamPoints {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [u8; 4] RGBA
    pub color: ByteVec,
    pub speed: u8,
}

// TE_BEAMENTPOINTS 1
pub struct TeBeamEntPoint {
    pub start_entity: i16,
    // [i16; 3]
    pub end_position: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: ByteVec,
    pub speed: u8,
}

// TE_GUNSHOT 2
pub struct TeGunShot {
    // [i16; 3]
    pub position: Vec<i16>,
}

// TE_EXPLOSION 3
pub struct TeExplosion {
    // [i16; 3]
    pub position: Vec<i16>,
    pub sprite_index: i16,
    pub scale: u8,
    pub frame_rame: u8,
    pub flags: u8,
}

// TE_TAREXPLOSION 4
pub struct TeTarExplosion {
    // [i16; 3]
    pub position: Vec<i16>,
}

// TE_SMOKE 5
pub struct TeSmoke {
    // [i16; 3]
    pub position: Vec<i16>,
    pub sprite_index: i16,
    pub scale: u8,
    pub frame_rate: u8,
}

// TE_TRACER 6
pub struct TeTracer {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
}

// TE_LIGHTNING 7
pub struct TeLightning {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    pub model_index: i16,
}

// TE_BEAMENTS 8
pub struct TeBeamEnts {
    // [i16; 3]
    pub start_entity: i16,
    pub end_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: ByteVec,
    pub speed: u8,
}

// TE_SPARKS 9
pub struct TeSparks {
    // [i16; 3]
    pub position: Vec<i16>,
}

// TE_LAVASPLASH 10
pub struct TeLavaSplash {
    // [i16; 3]
    pub position: Vec<i16>,
}

// TE_TELEPORT 11
pub struct TeTeleport {
    // [i16; 3]
    pub position: Vec<i16>,
}

// TE_EXPLOSION2 12
pub struct TeExplosion2 {
    // [i16; 3]
    pub position: Vec<i16>,
    pub color: u8,
    pub count: u8,
}

// TE_BSPDECAL 13
pub struct TeBspDecal {
    // [u8; 8]
    pub unknown1: ByteVec,
    pub entity_index: i16,
    // [u8; 2]
    pub unknown2: Option<ByteVec>,
}

// TE_IMPLOSION 14
pub struct TeImplosion {
    // [i16; 3]
    pub position: Vec<i16>,
    pub radius: u8,
    pub count: u8,
    pub life: u8,
}

// TE_SPRITETRAIL 15
pub struct TeSpriteTrail {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
    pub sprite_index: i16,
    pub count: u8,
    pub life: u8,
    pub scale: u8,
    pub velocity: u8,
    pub velocity_randomness: u8,
}

// TE_SPRITE 16
pub struct TeSprite {
    // [i16; 3]
    pub position: Vec<i16>,
    pub sprite_index: i16,
    pub scale: u8,
    pub brightness: u8,
}

// TE_BEAMSPRITE 18
pub struct TeBeamSprite {
    // [i16; 3]
    pub start_position: Vec<i16>,
    pub end_position: Vec<i16>,
    pub beam_sprite_index: i16,
    pub end_sprite_index: i16,
}

// TE_BEAMTORUS 19
pub struct TeBeamTorus {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub axis: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: ByteVec,
    pub speed: u8,
}

// TE_BEAMDISK 20
pub struct TeBeamDisk {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub axis: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: ByteVec,
    pub speed: u8,
}

// TE_BEAMCYLINDER 21
pub struct TeBeamCylinder {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub axis: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: ByteVec,
    pub speed: u8,
}

// TE_BEAMFOLLOW 22
pub struct TeBeamFollow {
    pub start_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    // [i16; 4] RGBA
    pub color: ByteVec,
}

// TE_GLOWSPRITE 23
pub struct TeGlowSprite {
    // [i16; 3]
    pub position: Vec<i16>,
    pub model_index: i16,
    pub scale: u8,
    pub size: u8,
    pub brightness: u8,
}

// TE_BEAMRING 24
pub struct TeBeamRing {
    pub start_entity: i16,
    pub end_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: ByteVec,
    pub speed: u8,
}

// TE_STREAKSPLASH 25
pub struct TeStreakSplash {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub vector: Vec<i16>,
    pub color: i16,
    pub count: u8,
    pub velocity: i16,
    pub velocity_randomness: i16,
}
// TE_DLIGHT 27
pub struct TeDLight {
    // [i16; 3]
    pub position: Vec<i16>,
    pub radius: u8,
    // [i16; 3]
    pub color: ByteVec,
    pub life: u8,
    pub decay_rate: u8,
}

// TE_ELIGHT 28
pub struct TeELight {
    pub entity_index: i16,
    // [i16; 3]
    pub position: Vec<i16>,
    pub radius: i16,
    // [i8; 3]
    pub color: ByteVec,
    pub life: u8,
    pub decay_rate: i16,
}
// TE_TEXTMESSAGE 29
pub struct TeTextMessage {
    pub channel: i8,
    pub x: i16,
    pub y: i16,
    pub effect: i8,
    // [u8; 4]
    pub text_color: ByteVec,
    // THE docs forgot to mention this
    pub effect_color: ByteVec,
    pub fade_in_time: i16,
    pub fade_out_time: i16,
    pub hold_time: i16,
    pub effect_time: Option<i16>,
    pub message: ByteVec,
}

// TE_LINE 30
pub struct TeLine {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
    pub life: i16,
    // [i8; 3]
    pub color: ByteVec,
}

// TE_BOX 31
pub struct TeBox {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
    pub life: i16,
    // [i8; 3]
    pub color: ByteVec,
}

// TE_KILLBEAM 99
pub struct TeKillBeam {
    pub entity_index: i16,
}

// TE_LARGEFUNNEL 100
pub struct TeLargeFunnel {
    // [i16; 3]
    pub start_position: Vec<i16>,
    pub entity_index: i16,
    pub flags: i16,
}

// TE_BLOODSTREAM 101
pub struct TeBloodStream {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub vector: i16,
    pub color: u8,
    pub count: u8,
}

// TE_SHOWLINE 102
pub struct TeShowLine {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
}

// TE_BLOOD 103
pub struct TeBlood {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub vector: i16,
    pub color: u8,
    pub count: u8,
}

// TE_DECAL 104
pub struct TeDecal {
    // [i16; 3]
    pub positiion: Vec<i16>,
    pub decal_index: u8,
    pub entity_index: i16,
}

// TE_FIZZ 105
pub struct TeFizz {
    pub entity_index: i16,
    pub model_index: i16,
    pub scale: u8,
}

// TE_MODEL 106
pub struct TeModel {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub velocity: Vec<i16>,
    pub angle_yaw: u8,
    pub model_index: i16,
    pub flags: u8,
    pub life: u8,
}

// TE_EXPLODEMODEL 107
pub struct TeExplodeModel {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub velocity: Vec<i16>,
    pub model_index: i16,
    pub count: i16,
    pub life: u8,
}

// TE_BREAKMODEL 108
pub struct TeBreakModel {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub size: Vec<i16>,
    // [i16; 3]
    pub velocity: Vec<i16>,
    pub velocity_randomness: u8,
    pub object_index: i16,
    pub count: u8,
    pub life: u8,
    pub flags: u8,
}

// TE_GUNSHOTDECAL 109
pub struct TeGunshotDecal {
    // [i16; 3]
    pub position: Vec<i16>,
    pub entity_index: i16,
    pub decal: u8,
}

// TE_SPRITESPRAY 110
pub struct TeSpriteSpray {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub velocity: Vec<i16>,
    pub model_index: i16,
    pub count: u8,
    pub speed: u8,
    pub random: u8,
}

// TE_ARMORRICOCHET 111
pub struct TeArmorRicochet {
    // [i16; 3]
    pub position: Vec<i16>,
    pub scale: u8,
}

// TE_PLAYERDECAL 112
pub struct TePlayerDecal {
    pub player_index: u8,
    // [i16; 3]
    pub position: Vec<i16>,
    pub entity_index: i16,
    pub decal_index: u8,
}

// TE_BUBBLES 113
pub struct TeBubbles {
    // [i16; 3]
    pub min_start_positition: Vec<i16>,
    // [i16; 3]
    pub max_start_position: Vec<i16>,
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub speed: i16,
}

// TE_BUBBLETRAIL 114
pub struct TeBubbleTrail {
    // [i16; 3]
    pub min_start_positition: Vec<i16>,
    // [i16; 3]
    pub max_start_position: Vec<i16>,
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub speed: i16,
}

// TE_BLOODSPRITE 115
pub struct TeBloodSprite {
    // [i16; 3]
    pub position: Vec<i16>,
    pub model_index: i16,
    pub decal_index: i16,
    pub color: u8,
    pub scale: u8,
}

// TE_WORLDDECAL 116
pub struct TeWorldDecal {
    // [i16; 3]
    pub position: Vec<i16>,
    pub texture_index: u8,
}

// TE_WORLDDECALHIGH 117
pub struct TeWorldDecalHigh {
    // [i16; 3]
    pub position: Vec<i16>,
    pub texture_index: u8,
}

// TE_DECALHIGH 118
pub struct TeDecalHigh {
    // [i16; 3]
    pub position: Vec<i16>,
    pub decal_index: u8,
    pub entity_index: i16,
}

// TE_PROJECTILE 119
pub struct TeProjectile {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub velocity: Vec<i16>,
    pub model_index: i16,
    pub life: u8,
    pub color: u8,
}

// TE_SPRAY 120
pub struct TeSpray {
    // [i16; 3]
    pub position: Vec<i16>,
    // [i16; 3]
    pub direction: Vec<i16>,
    pub model_index: i16,
    pub count: u8,
    pub life: u8,
    pub owner: u8,
}

// TE_PLAYERSPRITES 121
pub struct TePlayerSprites {
    pub entity_index: i16,
    pub model_index: i16,
    pub count: u8,
    pub variance: u8,
}

// TE_PARTICLEBURST 122
pub struct TeParticleBurst {
    // [i16; 3]
    pub origin: Vec<i16>,
    pub scale: i16,
    pub color: u8,
    pub duration: u8,
}

// TE_FIREFIELD 123
pub struct TeFireField {
    // [i16; 3]
    pub origin: Vec<i16>,
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub flags: u8,
    pub duration: u8,
}

// TE_PLAYERATTACHMENT 124
pub struct TePlayerAttachment {
    pub entity_index: u8,
    pub scale: i16,
    pub model_index: i16,
    pub life: i16,
}

// TE_KILLPLAYERATTACHMENT 125
pub struct TeKillPlayerAttachment {
    pub entity_index: u8,
}

// TE_MULTIGUNSHOT 126
pub struct TeMultigunShot {
    // [i16; 3]
    pub origin: Vec<i16>,
    // [i16; 3]
    pub direction: Vec<i16>,
    // [i16; 2]
    pub noise: Vec<i16>,
    pub count: u8,
    pub decal_index: u8,
}

// TE_USERTRACER 127
pub struct TeUserTracer {
    // [i16; 3]
    pub origin: Vec<i16>,
    // [i16; 3]
    pub velocity: Vec<i16>,
    pub life: u8,
    pub color: u8,
    pub scale: u8,
}

/// SVC_SETPAUSE 24
pub struct SvcSetPause {
    pub is_paused: i8,
}

/// SVC_SIGNONNUM 25
pub struct SvcSignOnNum {
    pub sign: i8,
}

/// SVC_CENTERPRINT 26
pub struct SvcCenterPrint {
    pub message: ByteVec,
}

/// SVC_KILLEDMONSTER 27
//

/// SVC_FOUNDSECRET 28
//

/// SVC_SPAWNSTATICSOUND 29
pub struct SvcSpawnStaticSound {
    // Vec3
    pub origin: Vec<i16>,
    pub sound_index: u16,
    pub volume: u8,
    pub attenuation: u8,
    pub entity_index: u16,
    pub pitch: u8,
    pub flags: u8,
}

/// SVC_INTERMISSION 30
//

/// SVC_FINALE 31
pub struct SvcFinale {
    pub text: ByteVec,
}

/// SVC_CDTRACK 32
pub struct SvcCdTrack {
    pub track: i8,
    pub loop_track: i8,
}

/// SVC_RESTORE 33
pub struct SvcRestore {
    pub save_name: ByteVec,
    pub map_count: u8,
    pub map_names: Vec<ByteVec>,
}

/// SVC_CUTSCENE 34
pub struct SvcCutscene {
    pub text: ByteVec,
}

/// SVC_WEAPONANIM 35
pub struct SvcWeaponAnim {
    pub sequence_number: i8,
    pub weapon_model_body_group: i8,
}

/// SVC_DECALNAME 36
pub struct SvcDecalName {
    pub position_index: u8,
    pub decal_name: ByteVec,
}

/// SVC_ROOMTYPE 37
pub struct SvcRoomType {
    pub room_type: u16,
}

/// SVC_ADDANGLE 38
pub struct SvcAddAngle {
    pub angle_to_add: i16,
}

/// SVC_NEWUSERMSG 39
#[derive(Debug, Clone)]
pub struct SvcNewUserMsg {
    pub index: u8,
    // weird but it's for consistency
    pub size: i8,
    // [u8; 16]
    pub name: ByteVec,
}

/// SVC_PACKETENTITIES 40
pub struct SvcPacketEntities {
    // [bool; 16]
    pub entity_count: BitVec,
    pub entity_states: Vec<EntityState>,
}

pub struct EntityState {
    pub entity_index: u16,
    pub increment_entity_number: bool,
    pub is_absolute_entity_index: Option<bool>,
    // [bool; 11]
    pub absolute_entity_index: Option<BitVec>,
    // [bool; 6]
    pub entity_index_difference: Option<BitVec>,
    pub has_custom_delta: bool,
    pub has_baseline_index: bool,
    // [bool; 6]
    pub baseline_index: Option<BitVec>,
    pub delta: Delta,
}

/// SVC_DELTAPACKETENTITIES 41
pub struct SvcDeltaPacketEntities {
    // [bool; 16]
    pub entity_count: BitVec,
    // [bool; 8]
    pub delta_sequence: BitVec,
    pub entity_states: Vec<EntityStateDelta>,
}

/// These infos are not like THE docs mention.
pub struct EntityStateDelta {
    // [bool; 11] but do u16 because arithmetic.
    pub entity_index: u16,
    pub remove_entity: bool,
    pub is_absolute_entity_index: bool,
    // [bool; 11]
    pub absolute_entity_index: Option<BitVec>,
    // [bool; 6]
    pub entity_index_difference: Option<BitVec>,
    // Need to be optional because if remove is true then it won't have delta.
    pub has_custom_delta: Option<bool>,
    pub delta: Option<Delta>,
}

/// SVC_CHOKE 42

/// SVC_RESOURCELIST 43
pub struct SvcResourceList {
    // [bool; 12]
    pub resource_count: BitVec,
    pub resources: Vec<Resource>,
    pub consistencies: Vec<Consistency>,
}

pub struct Resource {
    // [bool; 4]
    pub type_: BitVec,
    // &'[u8]
    pub name: BitVec,
    // [bool; 12]
    pub index: BitVec,
    // [bool; 24]
    pub size: BitVec,
    // [bool; 3]
    pub flags: BitVec,
    // [bool; 128]
    pub md5_hash: Option<BitVec>,
    pub has_extra_info: bool,
    // [bool; 256]
    pub extra_info: Option<BitVec>,
}

pub struct Consistency {
    pub has_check_file_flag: bool,
    pub is_short_index: Option<bool>,
    // [bool; 5]
    pub short_index: Option<BitVec>,
    // [bool; 10]
    pub long_index: Option<BitVec>,
}

/// SVC_NEWMOVEVARS 44
pub struct SvcNewMoveVars {
    pub gravity: f32,
    pub stop_speed: f32,
    pub max_speed: f32,
    pub spectator_max_speed: f32,
    pub accelerate: f32,
    pub airaccelerate: f32,
    pub water_accelerate: f32,
    pub friction: f32,
    pub edge_friction: f32,
    pub water_friction: f32,
    pub ent_garvity: f32,
    pub bounce: f32,
    pub step_size: f32,
    pub max_velocity: f32,
    pub z_max: f32,
    pub wave_height: f32,
    pub footsteps: i32,
    pub roll_angle: f32,
    pub roll_speed: f32,
    // Vec3
    pub sky_color: Vec<f32>,
    // Vec3
    pub sky_vec: Vec<f32>,
    pub sky_name: ByteVec,
}

/// SVC_RESOURCEREQUEST 45
pub struct SvcResourceRequest {
    pub spawn_count: i32,
    pub unknown: Vec<u8>,
}

/// SVC_CUSTOMIZATION 46
pub struct SvcCustomization {
    pub player_index: u8,
    pub type_: u8,
    pub name: ByteVec,
    pub index: u16,
    pub download_size: u32,
    pub flags: u8,
    // [u8; 16]
    pub md5_hash: Option<ByteVec>,
}

/// SVC_CROSSHAIRANGLE 47
pub struct SvcCrosshairAngle {
    pub pitch: i16,
    pub yaw: i16,
}

/// SVC_SOUNDFADE 48
pub struct SvcSoundFade {
    pub initial_percent: u8,
    pub hold_time: u8,
    pub fade_out_time: u8,
    pub fade_in_time: u8,
}

/// SVC_FILETXFERFAILED 49
pub struct SvcFileTxferFailed {
    pub file_name: ByteVec,
}

/// SVC_HLTV 50
pub struct SvcHltv {
    pub mode: u8,
}

/// SVC_DIRECTOR 51
pub struct SvcDirector {
    pub length: u8,
    pub flag: u8,
    pub message: ByteVec,
}

/// SVC_VOINCEINIT 52
pub struct SvcVoiceInit {
    pub codec_name: ByteVec,
    pub quality: i8,
}

/// SVC_VOICEDATA 53
pub struct SvcVoiceData {
    pub player_index: u8,
    pub size: u16,
    pub data: ByteVec,
}

/// SVC_SENDEXTRAINFO 54
pub struct SvcSendExtraInfo {
    pub fallback_dir: ByteVec,
    pub can_cheat: u8,
}

/// SVC_TIMESCALE 55
pub struct SvcTimeScale {
    pub time_scale: f32,
}

/// SVC_RESOURCELOCATION 56
pub struct SvcResourceLocation {
    pub download_url: ByteVec,
}

/// SVC_SENDCVARVALUE 57
pub struct SvcSendCvarValue {
    pub name: ByteVec,
}

/// SVC_SENDCVARVALUE2 58
pub struct SvcSendCvarValue2 {
    pub request_id: u32,
    pub name: ByteVec,
}
