use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bitvec::{order::Lsb0, slice::BitSlice as _BitSlice, vec::BitVec as _BitVec};

use crate::utils::get_initial_delta;

/// Auxillary data required for parsing/writing certain messages.
///
/// This includes delta decoders, custom messages, and max client
///
/// Basically storing some global values for demo to parse
#[derive(Clone, Debug)]
pub struct Aux {
    pub delta_decoders: DeltaDecoderTable,
    pub max_client: u8,
    pub custom_messages: CustomMessage,

    /// True if the demo was recorded by an HLTV client, false otherwise.
    ///
    /// HLTV clients can receive different data from the game server for messages like
    /// [SvcClientData], which affects parsing.
    pub is_hltv: bool,
}

impl Aux {
    pub fn new_raw() -> Self {
        Self {
            delta_decoders: get_initial_delta(),
            max_client: 1,
            custom_messages: CustomMessage::new(),
            is_hltv: false,
        }
    }

    pub fn new2() -> AuxRefCell {
        Rc::new(RefCell::new(Self::new_raw()))
    }
}

pub type AuxRefCell = Rc<RefCell<Aux>>;

// Everything not related to netmessage starts here
#[derive(Debug, Clone)]
pub struct Demo {
    pub header: Header,
    pub directory: Directory,
    /// Not part of a demo. Do not use this
    pub _aux: Option<AuxRefCell>,
}

#[derive(Debug, Clone)]
pub struct Header {
    /// `[u8; 8]`
    pub magic: Vec<u8>,
    pub demo_protocol: i32,
    pub network_protocol: i32,
    /// `[u8; 260]`
    pub map_name: Vec<u8>,
    /// `[u8; 260]`
    pub game_directory: Vec<u8>,
    pub map_checksum: u32,
    pub directory_offset: i32,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub entries: Vec<DirectoryEntry>,
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub type_: i32,
    /// `[u8; 64]`
    pub description: Vec<u8>,
    pub flags: i32,
    pub cd_track: i32,
    pub track_time: f32,
    pub frame_count: i32,
    pub frame_offset: i32,
    pub file_length: i32,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub time: f32,
    pub frame: i32,
    pub frame_data: FrameData,
}

#[derive(Debug, Clone)]
pub enum FrameData {
    NetworkMessage(Box<(NetworkMessageType, NetworkMessage)>),
    DemoStart,
    ConsoleCommand(ConsoleCommand),
    ClientData(ClientData),
    NextSection,
    Event(Event),
    WeaponAnimation(WeaponAnimation),
    Sound(Sound),
    DemoBuffer(DemoBuffer),
}

#[derive(Debug, Clone)]
pub struct ConsoleCommand {
    /// `[u8; 64]`
    pub command: Vec<u8>,
}

/// `[T; 3]`
type Point<T> = Vec<T>;

#[derive(Debug, Clone)]
pub struct ClientData {
    pub origin: Point<f32>,
    pub viewangles: Point<f32>,
    pub weapon_bits: i32,
    pub fov: f32,
}

/// This is different from [`EventS`], which is used for event types in netmessage
#[derive(Debug, Clone)]
pub struct Event {
    pub flags: i32,
    pub index: i32,
    pub delay: f32,
    pub args: EventArgs,
}

#[derive(Debug, Clone)]
pub struct EventArgs {
    pub flags: i32,
    pub entity_index: i32,
    pub origin: Point<f32>,
    pub angles: Point<f32>,
    pub velocity: Point<f32>,
    pub ducking: i32,
    pub fparam1: f32,
    pub fparam2: f32,
    pub iparam1: i32,
    pub iparam2: i32,
    pub bparam1: i32,
    pub bparam2: i32,
}

#[derive(Debug, Clone)]
pub struct Sound {
    pub channel: i32,
    /// `[u8; sample_length]`
    pub sample: Vec<u8>,
    pub attenuation: f32,
    pub volume: f32,
    pub flags: i32,
    pub pitch: i32,
}

#[derive(Debug, Clone)]
pub struct WeaponAnimation {
    pub anim: i32,
    pub body: i32,
}

#[derive(Debug, Clone)]
pub struct DemoBuffer {
    /// `[u8; buffer_length]`
    pub buffer: Vec<u8>,
}

/// <https://github.com/YaLTeR/hldemo-rs/blob/cbc1efa212a4fc49c776304058efd07e0369caa7/src/types.rs#L187>
#[derive(Debug, Clone)]
pub enum NetworkMessageType {
    Start,
    Normal,
    Unknown(u8),
}

impl TryFrom<u8> for NetworkMessageType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Start),
            1 => Ok(Self::Normal),
            2..=9 => Err("network message type cannot overlap with other messages"),
            rest => Ok(Self::Unknown(rest)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkMessage {
    pub info: DemoInfo,
    pub sequence_info: SequenceInfo,
    // need this so the messages type can be [`Parsed`] or [`Unparsed`]
    pub message_length: u32,
    pub messages: MessageData,
}

#[derive(Debug, Clone)]
pub struct DemoInfo {
    pub timestamp: f32,
    pub refparams: RefParams,
    pub usercmd: UserCmd,
    pub movevars: MoveVars,
    pub view: Point<f32>,
    pub viewmodel: i32,
}

#[derive(Debug, Clone)]
pub struct RefParams {
    pub view_origin: Point<f32>,
    pub view_angles: Point<f32>,
    pub forward: Point<f32>,
    pub right: Point<f32>,
    pub up: Point<f32>,
    pub frame_time: f32,
    pub time: f32,
    pub intermission: i32,
    pub paused: i32,
    pub spectator: i32,
    pub on_ground: i32,
    pub water_level: i32,
    pub sim_vel: Point<f32>,
    pub sim_org: Point<f32>,
    pub view_height: Point<f32>,
    pub ideal_pitch: f32,
    pub cl_viewangles: Point<f32>,
    pub health: i32,
    pub crosshair_angle: Point<f32>,
    pub view_size: f32,
    pub punch_angle: Point<f32>,
    pub max_clients: i32,
    pub view_entity: i32,
    pub player_num: i32,
    pub max_entities: i32,
    pub demo_playback: i32,
    pub hardware: i32,
    pub smoothing: i32,
    pub ptr_cmd: i32,
    pub ptr_move_vars: i32,
    /// `[i32; 4]`
    pub view_port: Vec<i32>,
    pub next_view: i32,
    pub only_client_draw: i32,
}

#[derive(Debug, Clone)]
pub struct UserCmd {
    pub lerp_msec: i16,
    pub msec: u8,
    pub unknown1: u8,
    pub view_angles: Point<f32>,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub light_level: i8,
    pub unknonwn2: u8,
    pub buttons: u16,
    pub impulse: i8,
    pub weapon_select: i8,
    pub unknown3: u8,
    pub unknown4: u8,
    pub impact_index: i32,
    pub impact_position: Point<f32>,
}

#[derive(Debug, Clone)]
pub struct MoveVars {
    pub gravity: f32,
    pub stopspeed: f32,
    pub maxspeed: f32,
    pub spectatormaxspeed: f32,
    pub accelerate: f32,
    pub airaccelerate: f32,
    pub wateraccelerate: f32,
    pub friction: f32,
    pub edgefriction: f32,
    pub waterfriction: f32,
    pub entgravity: f32,
    pub bounce: f32,
    pub stepsize: f32,
    pub maxvelocity: f32,
    pub zmax: f32,
    pub wave_height: f32,
    pub footsteps: i32,
    /// `[u8; 32]`
    pub sky_name: Vec<u8>,
    pub rollangle: f32,
    pub rollspeed: f32,
    pub skycolor: Point<f32>,
    pub skyvec: Point<f32>,
}

#[derive(Debug, Clone)]
pub struct SequenceInfo {
    pub incoming_sequence: i32,
    pub incoming_acknowledged: i32,
    pub incoming_reliable_acknowledged: i32,
    pub incoming_reliable_sequence: i32,
    pub outgoing_sequence: i32,
    pub reliable_sequence: i32,
    pub last_reliable_sequence: i32,
}

#[derive(Debug, Clone)]
pub enum MessageData {
    Parsed(Vec<NetMessage>),
    Raw(Vec<u8>),
}

// Everything related to netmessage starts here

// Primitive
// pub type BitVec = _BitVec<u8>;
pub type BitVec = _BitVec<u8, Lsb0>;
pub type BitSlice = _BitSlice<u8, Lsb0>;
pub type ByteVec = Vec<u8>;

// Delta
pub type Delta = HashMap<String, ByteVec>;
pub type DeltaDecoder = Vec<DeltaDecoderS>;
pub type DeltaDecoderTable = HashMap<String, DeltaDecoder>;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum NetMessage {
    UserMessage(UserMessage),
    EngineMessage(Box<EngineMessage>),
}

pub type CustomMessage = HashMap<u8, SvcNewUserMsg>;
#[derive(Debug, Clone)]
pub struct UserMessage {
    pub id: u8,
    /// `[bool; 16]`
    pub name: ByteVec,
    pub data: ByteVec,
}

// Messages
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum EngineMessage {
    SvcBad = 0,
    SvcNop = 1,
    SvcDisconnect(SvcDisconnect) = 2,
    SvcEvent(SvcEvent) = 3,
    SvcVersion(SvcVersion) = 4,
    SvcSetView(SvcSetView) = 5,
    SvcSound(Box<SvcSound>) = 6,
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
    SvcNewMovevars(SvcNewMovevars) = 44,
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

// SVC_BAD 0

// SVC_NOP 1

/// SVC_DISCONNECT 2
#[derive(Debug, Clone)]
pub struct SvcDisconnect {
    pub reason: ByteVec,
}

/// SVC_EVENT 3
#[derive(Debug, Clone)]
pub struct SvcEvent {
    /// `[bool; 5]`
    pub event_count: BitVec,
    pub events: Vec<EventS>,
}
#[derive(Debug, Clone)]
pub struct EventS {
    /// `[bool; 10]`
    pub event_index: BitVec,
    pub has_packet_index: bool,
    /// `[bool; 11]`
    pub packet_index: Option<BitVec>,
    pub has_delta: Option<bool>,
    pub delta: Option<Delta>,
    pub has_fire_time: bool,
    /// `[bool; 16]`
    pub fire_time: Option<BitVec>,
}

/// SVC_VERSION 4
#[derive(Debug, Clone)]
pub struct SvcVersion {
    pub protocol_version: u32,
}

/// SVC_SETVIEW 5
#[derive(Debug, Clone)]
pub struct SvcSetView {
    pub entity_index: i16,
}

/// SVC_SOUND 6
#[derive(Debug, Clone)]
pub struct SvcSound {
    /// `[bool; 9]`
    pub flags: BitVec,
    pub volume: Option<BitVec>,
    pub attenuation: Option<BitVec>,
    /// `[bool; 3]`
    pub channel: BitVec,
    /// `[bool; 11]`
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
#[derive(Debug, Clone)]
pub struct OriginCoord {
    pub int_flag: bool,
    pub fraction_flag: bool,
    pub is_negative: Option<bool>,
    /// `[bool; 12]`
    pub int_value: Option<BitVec>,
    /// `[bool; 3]`
    pub fraction_value: Option<BitVec>,
    // There is no unknow, Xd
    // `[bool; 2]`
    // pub unknown: BitVec,
}

/// SVC_TIME 7
#[derive(Debug, Clone)]
pub struct SvcTime {
    pub time: f32,
}

/// SVC_PRINT 8
#[derive(Debug, Clone)]
pub struct SvcPrint {
    pub message: ByteVec,
}

/// SVC_STUFFTEXT 9
#[derive(Debug, Clone)]
pub struct SvcStuffText {
    pub command: ByteVec,
}

/// SVC_SETANGLE 10
#[derive(Debug, Clone)]
pub struct SvcSetAngle {
    pub pitch: i16,
    pub yaw: i16,
    pub roll: i16,
}

/// SVC_SERVERINFO 11
#[derive(Debug, Clone)]
pub struct SvcServerInfo {
    pub protocol: i32,
    pub spawn_count: i32,
    pub map_checksum: i32,
    /// `[u8; 16]`
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
#[derive(Debug, Clone)]
pub struct SvcLightStyle {
    pub index: u8,
    pub light_info: ByteVec,
}

/// SVC_UPDATEUSERINFO 13
#[derive(Debug, Clone)]
pub struct SvcUpdateUserInfo {
    pub index: u8,
    pub id: u32,
    pub user_info: ByteVec,
    /// `[u8; 16]`
    pub cd_key_hash: ByteVec,
}

/// SVC_DELTADESCRIPTION 14
#[derive(Debug, Clone)]
pub struct SvcDeltaDescription {
    pub name: ByteVec,
    pub total_fields: u16,
    pub fields: DeltaDecoder,
    pub clone: ByteVec,
}

/// SVC_CLIENTDATA 15
#[derive(Debug, Clone)]
pub struct SvcClientData {
    pub has_delta_update_mask: bool,
    /// `[bool; 8]`
    pub delta_update_mask: Option<BitVec>,
    pub client_data: Delta,
    pub weapon_data: Option<Vec<ClientDataWeaponData>>,
}
#[derive(Debug, Clone)]
pub struct ClientDataWeaponData {
    /// `[bool; 6]`
    pub weapon_index: BitVec,
    pub weapon_data: Delta,
}

/// SVC_STOPSOUND 16
#[derive(Debug, Clone)]
pub struct SvcStopSound {
    pub entity_index: i16,
}

/// SVC_PINGS 17
#[derive(Debug, Clone)]
pub struct SvcPings {
    pub pings: Vec<PingS>,
}
#[derive(Debug, Clone)]
pub struct PingS {
    pub has_ping_data: bool,
    pub player_id: Option<u8>,
    pub ping: Option<u8>,
    pub loss: Option<u8>,
}

/// SVC_PARTICLE 18
#[derive(Debug, Clone)]
pub struct SvcParticle {
    /// Vec3
    pub origin: Vec<i16>,
    /// Vec3
    pub direction: ByteVec,
    pub count: u8,
    pub color: u8,
}

// SVC_PARTICLE 19

/// SVC_SPAWNSTATIC 20
#[derive(Debug, Clone)]
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
    /// `[u8; 3]`
    pub render_color: Option<ByteVec>,
}

/// SVC_EVENTRELIABLE 21
#[derive(Debug, Clone)]
pub struct SvcEventReliable {
    /// `[bool; 10]`
    pub event_index: BitVec,
    pub event_args: Delta,
    pub has_fire_time: bool,
    /// `[bool; 16]`
    pub fire_time: Option<BitVec>,
}

/// SVC_SPAWNBASELINE 22
#[derive(Debug, Clone)]
pub struct SvcSpawnBaseline {
    pub entities: Vec<EntityS>,
    // These members are not inside EntityS like cgdangelo/talent suggests.
    /// `[bool; 6]`
    pub total_extra_data: BitVec,
    pub extra_data: Vec<Delta>,
}
#[derive(Debug, Clone)]
pub struct EntityS {
    // Goodies
    pub entity_index: u16,
    /// `[bool; 11]`
    pub index: BitVec,
    /// `[bool; 2]`
    pub type_: BitVec,
    // One delta for 3 types
    pub delta: Delta,
}

/// SVC_TEMPENTITY 23
#[derive(Debug, Clone)]
pub struct SvcTempEntity {
    pub entity_type: u8,
    pub entity: TempEntity,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum TempEntity {
    /// `[u8; 24]`
    TeBeamPoints(TeBeamPoints) = 0,
    /// `[u8; 20]`
    TeBeamEntPoint(ByteVec) = 1,
    /// `[u8; 6]`
    TeGunshot(ByteVec) = 2,
    // It is 11
    /// `[u8; 11]`
    TeExplosion(ByteVec) = 3,
    /// `[u8; 6]`
    TeTarExplosion(ByteVec) = 4,
    /// `[u8; 10]`
    TeSmoke(ByteVec) = 5,
    /// `[u8; 12]`
    TeTracer(ByteVec) = 6,
    /// `[u8; 17]`
    TeLightning(ByteVec) = 7,
    /// `[u8; 16]`
    TeBeamEnts(ByteVec) = 8,
    /// `[u8; 6]`
    TeSparks(ByteVec) = 9,
    /// `[u8; 6]`
    TeLavaSplash(ByteVec) = 10,
    /// `[u8; 6]`
    TeTeleport(ByteVec) = 11,
    /// `[u8; 8]`
    TeExplosion2(ByteVec) = 12,
    TeBspDecal(TeBspDecal) = 13,
    /// `[u8; 9]`
    TeImplosion(ByteVec) = 14,
    /// `[u8; 19]`
    TeSpriteTrail(ByteVec) = 15,
    /// `[u8; 10]`
    TeSprite(ByteVec) = 17,
    /// `[u8; 16]`
    TeBeamSprite(ByteVec) = 18,
    /// `[u8; 24]`
    TeBeamTorus(ByteVec) = 19,
    /// `[u8; 24]`
    TeBeamDisk(ByteVec) = 20,
    /// `[u8; 24]`
    TeBeamCylinder(ByteVec) = 21,
    /// `[u8; 10]`
    TeBeamFollow(ByteVec) = 22,
    /// `[u8; 11]`
    TeGlowSprite(ByteVec) = 23,
    /// `[u8; 16]`
    TeBeamRing(ByteVec) = 24,
    /// `[u8; 19]`
    TeStreakSplash(ByteVec) = 25,
    /// `[u8; 12]`
    TeDLight(ByteVec) = 27,
    /// `[u8; 16]`
    TeELight(ByteVec) = 28,
    TeTextMessage(TeTextMessage) = 29,
    /// `[u8; 17]`
    TeLine(ByteVec) = 30,
    /// `[u8; 17]`
    TeBox(ByteVec) = 31,
    /// `[u8; 2]`
    TeKillBeam(ByteVec) = 99,
    /// `[u8; 10]`
    TeLargeFunnel(ByteVec) = 100,
    /// `[u8; 14]`
    TeBloodStream(ByteVec) = 101,
    /// `[u8; 12]`
    TeShowLine(ByteVec) = 102,
    /// `[u8; 14]`
    TeBlood(ByteVec) = 103,
    /// `[u8; 9]`
    TeDecal(ByteVec) = 104,
    /// `[u8; 5]`
    TeFizz(ByteVec) = 105,
    /// `[u8; 17]`
    TeModel(ByteVec) = 106,
    /// `[u8; 13]`
    TeExplodeModel(ByteVec) = 107,
    // It is 24
    /// `[u8; 24]`
    TeBreakModel(ByteVec) = 108,
    /// `[u8; 9]`
    TeGunshotDecal(ByteVec) = 109,
    /// `[u8; 17]`
    TeSpriteSpray(ByteVec) = 110,
    /// `[u8; 7]`
    TeArmorRicochet(ByteVec) = 111,
    /// `[u8; 10]`
    TePlayerDecal(ByteVec) = 112,
    /// `[u8; 10]`
    TeBubbles(ByteVec) = 113,
    /// `[u8; 19]`
    TeBubbleTrail(ByteVec) = 114,
    /// `[u8; 12]`
    TeBloodSprite(ByteVec) = 115,
    /// `[u8; 7]`
    TeWorldDecal(ByteVec) = 116,
    /// `[u8; 7]`
    TeWorldDecalHigh(ByteVec) = 117,
    /// `[u8; 9]`
    TeDecalHigh(ByteVec) = 118,
    /// `[u8; 16]`
    TeProjectile(ByteVec) = 119,
    /// `[u8; 18]`
    TeSpray(ByteVec) = 120,
    /// `[u8; 5]`
    TePlayerSprites(ByteVec) = 121,
    /// `[u8; 10]`
    TeParticleBurst(ByteVec) = 122,
    /// `[u8; 9]`
    TeFireField(ByteVec) = 123,
    /// `[u8; 7]`
    TePlayerAttachment(ByteVec) = 124,
    /// `[u8; 1]`
    TeKillPlayerAttachment(ByteVec) = 125,
    // It is 18.
    /// `[u8; 18]`
    TeMultigunShot(ByteVec) = 126,
    /// `[u8; 15]`
    TeUserTracer(ByteVec) = 127,
}

impl TempEntity {
    pub fn id(&self) -> u8 {
        match self {
            TempEntity::TeBeamPoints(_) => 0,
            TempEntity::TeBeamEntPoint(_) => 1,
            TempEntity::TeGunshot(_) => 2,
            TempEntity::TeExplosion(_) => 3,
            TempEntity::TeTarExplosion(_) => 4,
            TempEntity::TeSmoke(_) => 5,
            TempEntity::TeTracer(_) => 6,
            TempEntity::TeLightning(_) => 7,
            TempEntity::TeBeamEnts(_) => 8,
            TempEntity::TeSparks(_) => 9,
            TempEntity::TeLavaSplash(_) => 10,
            TempEntity::TeTeleport(_) => 11,
            TempEntity::TeExplosion2(_) => 12,
            TempEntity::TeBspDecal(_) => 13,
            TempEntity::TeImplosion(_) => 14,
            TempEntity::TeSpriteTrail(_) => 15,
            TempEntity::TeSprite(_) => 17,
            TempEntity::TeBeamSprite(_) => 18,
            TempEntity::TeBeamTorus(_) => 19,
            TempEntity::TeBeamDisk(_) => 20,
            TempEntity::TeBeamCylinder(_) => 21,
            TempEntity::TeBeamFollow(_) => 22,
            TempEntity::TeGlowSprite(_) => 23,
            TempEntity::TeBeamRing(_) => 24,
            TempEntity::TeStreakSplash(_) => 25,
            TempEntity::TeDLight(_) => 27,
            TempEntity::TeELight(_) => 28,
            TempEntity::TeTextMessage(_) => 29,
            TempEntity::TeLine(_) => 30,
            TempEntity::TeBox(_) => 31,
            TempEntity::TeKillBeam(_) => 99,
            TempEntity::TeLargeFunnel(_) => 100,
            TempEntity::TeBloodStream(_) => 101,
            TempEntity::TeShowLine(_) => 102,
            TempEntity::TeBlood(_) => 103,
            TempEntity::TeDecal(_) => 104,
            TempEntity::TeFizz(_) => 105,
            TempEntity::TeModel(_) => 106,
            TempEntity::TeExplodeModel(_) => 107,
            TempEntity::TeBreakModel(_) => 108,
            TempEntity::TeGunshotDecal(_) => 109,
            TempEntity::TeSpriteSpray(_) => 110,
            TempEntity::TeArmorRicochet(_) => 111,
            TempEntity::TePlayerDecal(_) => 112,
            TempEntity::TeBubbles(_) => 113,
            TempEntity::TeBubbleTrail(_) => 114,
            TempEntity::TeBloodSprite(_) => 115,
            TempEntity::TeWorldDecal(_) => 116,
            TempEntity::TeWorldDecalHigh(_) => 117,
            TempEntity::TeDecalHigh(_) => 118,
            TempEntity::TeProjectile(_) => 119,
            TempEntity::TeSpray(_) => 120,
            TempEntity::TePlayerSprites(_) => 121,
            TempEntity::TeParticleBurst(_) => 122,
            TempEntity::TeFireField(_) => 123,
            TempEntity::TePlayerAttachment(_) => 124,
            TempEntity::TeKillPlayerAttachment(_) => 125,
            TempEntity::TeMultigunShot(_) => 126,
            TempEntity::TeUserTracer(_) => 127,
        }
    }
}

/// TE_BEAMPOINTS 0
#[derive(Debug, Clone)]
pub struct TeBeamPoints {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    /// `[u8; 4]` RGBA
    pub color: ByteVec,
    pub speed: u8,
}

/// TE_BEAMENTPOINTS 1
#[derive(Debug, Clone)]
pub struct TeBeamEntPoint {
    pub start_entity: i16,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    /// `[i16; 4]` RGBA
    pub color: ByteVec,
    pub speed: u8,
}

/// TE_GUNSHOT 2
#[derive(Debug, Clone)]
pub struct TeGunShot {
    /// `[i16; 3]`
    pub position: Vec<i16>,
}

/// TE_EXPLOSION 3
#[derive(Debug, Clone)]
pub struct TeExplosion {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub sprite_index: i16,
    pub scale: u8,
    pub frame_rame: u8,
    pub flags: u8,
}

/// TE_TAREXPLOSION 4
#[derive(Debug, Clone)]
pub struct TeTarExplosion {
    /// `[i16; 3]`
    pub position: Vec<i16>,
}

/// TE_SMOKE 5
#[derive(Debug, Clone)]
pub struct TeSmoke {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub sprite_index: i16,
    pub scale: u8,
    pub frame_rate: u8,
}

/// TE_TRACER 6
#[derive(Debug, Clone)]
pub struct TeTracer {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
}

/// TE_LIGHTNING 7
#[derive(Debug, Clone)]
pub struct TeLightning {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    pub model_index: i16,
}

/// TE_BEAMENTS 8
#[derive(Debug, Clone)]
pub struct TeBeamEnts {
    /// `[i16; 3]`
    pub start_entity: i16,
    pub end_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    /// `[i16; 4]` RGBA
    pub color: ByteVec,
    pub speed: u8,
}

/// TE_SPARKS 9
#[derive(Debug, Clone)]
pub struct TeSparks {
    /// `[i16; 3]`
    pub position: Vec<i16>,
}

/// TE_LAVASPLASH 10
#[derive(Debug, Clone)]
pub struct TeLavaSplash {
    /// `[i16; 3]`
    pub position: Vec<i16>,
}

/// TE_TELEPORT 11
#[derive(Debug, Clone)]
pub struct TeTeleport {
    /// `[i16; 3]`
    pub position: Vec<i16>,
}

/// TE_EXPLOSION2 12
#[derive(Debug, Clone)]
pub struct TeExplosion2 {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub color: u8,
    pub count: u8,
}

/// TE_BSPDECAL 13
#[derive(Debug, Clone)]
pub struct TeBspDecal {
    /// `[u8; 8]`
    pub unknown1: ByteVec,
    pub entity_index: i16,
    /// `[u8; 2]`
    pub unknown2: Option<ByteVec>,
}

/// TE_IMPLOSION 14
#[derive(Debug, Clone)]
pub struct TeImplosion {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub radius: u8,
    pub count: u8,
    pub life: u8,
}

/// TE_SPRITETRAIL 15
#[derive(Debug, Clone)]
pub struct TeSpriteTrail {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
    pub sprite_index: i16,
    pub count: u8,
    pub life: u8,
    pub scale: u8,
    pub velocity: u8,
    pub velocity_randomness: u8,
}

/// TE_SPRITE 17
#[derive(Debug, Clone)]
pub struct TeSprite {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub sprite_index: i16,
    pub scale: u8,
    pub brightness: u8,
}

/// TE_BEAMSPRITE 18
#[derive(Debug, Clone)]
pub struct TeBeamSprite {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    pub end_position: Vec<i16>,
    pub beam_sprite_index: i16,
    pub end_sprite_index: i16,
}

/// TE_BEAMTORUS 19
#[derive(Debug, Clone)]
pub struct TeBeamTorus {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub axis: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    /// `[i16; 4] RGBA`
    pub color: ByteVec,
    pub speed: u8,
}

/// TE_BEAMDISK 20
#[derive(Debug, Clone)]
pub struct TeBeamDisk {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub axis: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    /// `[i16; 4]` RGBA
    pub color: ByteVec,
    pub speed: u8,
}

/// TE_BEAMCYLINDER 21
#[derive(Debug, Clone)]
pub struct TeBeamCylinder {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub axis: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    /// `[i16; 4]` RGBA
    pub color: ByteVec,
    pub speed: u8,
}

/// TE_BEAMFOLLOW 22
#[derive(Debug, Clone)]
pub struct TeBeamFollow {
    pub start_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    /// `[i16; 4]` RGBA
    pub color: ByteVec,
}

/// TE_GLOWSPRITE 23
#[derive(Debug, Clone)]
pub struct TeGlowSprite {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub model_index: i16,
    pub scale: u8,
    pub size: u8,
    pub brightness: u8,
}

/// TE_BEAMRING 24
#[derive(Debug, Clone)]
pub struct TeBeamRing {
    pub start_entity: i16,
    pub end_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    /// `[i16; 4]` RGBA
    pub color: ByteVec,
    pub speed: u8,
}

/// TE_STREAKSPLASH 25
#[derive(Debug, Clone)]
pub struct TeStreakSplash {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub vector: Vec<i16>,
    pub color: i16,
    pub count: u8,
    pub velocity: i16,
    pub velocity_randomness: i16,
}
/// TE_DLIGHT 27
#[derive(Debug, Clone)]
pub struct TeDLight {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub radius: u8,
    /// `[i16; 3]`
    pub color: ByteVec,
    pub life: u8,
    pub decay_rate: u8,
}

/// TE_ELIGHT 28
#[derive(Debug, Clone)]
pub struct TeELight {
    pub entity_index: i16,
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub radius: i16,
    /// `[i8; 3]`
    pub color: ByteVec,
    pub life: u8,
    pub decay_rate: i16,
}
/// TE_TEXTMESSAGE 29
#[derive(Debug, Clone)]
pub struct TeTextMessage {
    pub channel: i8,
    pub x: i16,
    pub y: i16,
    pub effect: i8,
    /// `[u8; 4]`
    pub text_color: ByteVec,
    // THE docs forgot to mention this
    pub effect_color: ByteVec,
    pub fade_in_time: i16,
    pub fade_out_time: i16,
    pub hold_time: i16,
    pub effect_time: Option<i16>,
    pub message: ByteVec,
}

/// TE_LINE 30
#[derive(Debug, Clone)]
pub struct TeLine {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
    pub life: i16,
    /// `[i8; 3]`
    pub color: ByteVec,
}

/// TE_BOX 31
#[derive(Debug, Clone)]
pub struct TeBox {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
    pub life: i16,
    /// `[i8; 3]`
    pub color: ByteVec,
}

/// TE_KILLBEAM 99
#[derive(Debug, Clone)]
pub struct TeKillBeam {
    pub entity_index: i16,
}

/// TE_LARGEFUNNEL 100
#[derive(Debug, Clone)]
pub struct TeLargeFunnel {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    pub entity_index: i16,
    pub flags: i16,
}

/// TE_BLOODSTREAM 101
#[derive(Debug, Clone)]
pub struct TeBloodStream {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub vector: i16,
    pub color: u8,
    pub count: u8,
}

/// TE_SHOWLINE 102
#[derive(Debug, Clone)]
pub struct TeShowLine {
    /// `[i16; 3]`
    pub start_position: Vec<i16>,
    /// `[i16; 3]`
    pub end_position: Vec<i16>,
}

/// TE_BLOOD 103
#[derive(Debug, Clone)]
pub struct TeBlood {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub vector: i16,
    pub color: u8,
    pub count: u8,
}

/// TE_DECAL 104
#[derive(Debug, Clone)]
pub struct TeDecal {
    /// `[i16; 3]`
    pub positiion: Vec<i16>,
    pub decal_index: u8,
    pub entity_index: i16,
}

/// TE_FIZZ 105
#[derive(Debug, Clone)]
pub struct TeFizz {
    pub entity_index: i16,
    pub model_index: i16,
    pub scale: u8,
}

/// TE_MODEL 106
#[derive(Debug, Clone)]
pub struct TeModel {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub velocity: Vec<i16>,
    pub angle_yaw: u8,
    pub model_index: i16,
    pub flags: u8,
    pub life: u8,
}

/// TE_EXPLODEMODEL 107
#[derive(Debug, Clone)]
pub struct TeExplodeModel {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub velocity: Vec<i16>,
    pub model_index: i16,
    pub count: i16,
    pub life: u8,
}

/// TE_BREAKMODEL 108
#[derive(Debug, Clone)]
pub struct TeBreakModel {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub size: Vec<i16>,
    /// `[i16; 3]`
    pub velocity: Vec<i16>,
    pub velocity_randomness: u8,
    pub object_index: i16,
    pub count: u8,
    pub life: u8,
    pub flags: u8,
}

/// TE_GUNSHOTDECAL 109
#[derive(Debug, Clone)]
pub struct TeGunshotDecal {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub entity_index: i16,
    pub decal: u8,
}

/// TE_SPRITESPRAY 110
#[derive(Debug, Clone)]
pub struct TeSpriteSpray {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub velocity: Vec<i16>,
    pub model_index: i16,
    pub count: u8,
    pub speed: u8,
    pub random: u8,
}

/// TE_ARMORRICOCHET 111
#[derive(Debug, Clone)]
pub struct TeArmorRicochet {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub scale: u8,
}

/// TE_PLAYERDECAL 112
#[derive(Debug, Clone)]
pub struct TePlayerDecal {
    pub player_index: u8,
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub entity_index: i16,
    pub decal_index: u8,
}

/// TE_BUBBLES 113
#[derive(Debug, Clone)]
pub struct TeBubbles {
    /// `[i16; 3]`
    pub min_start_positition: Vec<i16>,
    /// `[i16; 3]`
    pub max_start_position: Vec<i16>,
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub speed: i16,
}

/// TE_BUBBLETRAIL 114
#[derive(Debug, Clone)]
pub struct TeBubbleTrail {
    /// `[i16; 3]`
    pub min_start_positition: Vec<i16>,
    /// `[i16; 3]`
    pub max_start_position: Vec<i16>,
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub speed: i16,
}

/// TE_BLOODSPRITE 115
#[derive(Debug, Clone)]
pub struct TeBloodSprite {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub model_index: i16,
    pub decal_index: i16,
    pub color: u8,
    pub scale: u8,
}

/// TE_WORLDDECAL 116
#[derive(Debug, Clone)]
pub struct TeWorldDecal {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub texture_index: u8,
}

/// TE_WORLDDECALHIGH 117
#[derive(Debug, Clone)]
pub struct TeWorldDecalHigh {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub texture_index: u8,
}

/// TE_DECALHIGH 118
#[derive(Debug, Clone)]
pub struct TeDecalHigh {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    pub decal_index: u8,
    pub entity_index: i16,
}

/// TE_PROJECTILE 119
#[derive(Debug, Clone)]
pub struct TeProjectile {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub velocity: Vec<i16>,
    pub model_index: i16,
    pub life: u8,
    pub color: u8,
}

/// TE_SPRAY 120
#[derive(Debug, Clone)]
pub struct TeSpray {
    /// `[i16; 3]`
    pub position: Vec<i16>,
    /// `[i16; 3]`
    pub direction: Vec<i16>,
    pub model_index: i16,
    pub count: u8,
    pub life: u8,
    pub owner: u8,
}

/// TE_PLAYERSPRITES 121
#[derive(Debug, Clone)]
pub struct TePlayerSprites {
    pub entity_index: i16,
    pub model_index: i16,
    pub count: u8,
    pub variance: u8,
}

/// TE_PARTICLEBURST 122
#[derive(Debug, Clone)]
pub struct TeParticleBurst {
    /// `[i16; 3]`
    pub origin: Vec<i16>,
    pub scale: i16,
    pub color: u8,
    pub duration: u8,
}

/// TE_FIREFIELD 123
#[derive(Debug, Clone)]
pub struct TeFireField {
    /// `[i16; 3]`
    pub origin: Vec<i16>,
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub flags: u8,
    pub duration: u8,
}

/// TE_PLAYERATTACHMENT 124
#[derive(Debug, Clone)]
pub struct TePlayerAttachment {
    pub entity_index: u8,
    pub scale: i16,
    pub model_index: i16,
    pub life: i16,
}

/// TE_KILLPLAYERATTACHMENT 125
#[derive(Debug, Clone)]
pub struct TeKillPlayerAttachment {
    pub entity_index: u8,
}

/// TE_MULTIGUNSHOT 126
#[derive(Debug, Clone)]
pub struct TeMultigunShot {
    /// `[i16; 3]`
    pub origin: Vec<i16>,
    /// `[i16; 3]`
    pub direction: Vec<i16>,
    /// `[i16; 2]`
    pub noise: Vec<i16>,
    pub count: u8,
    pub decal_index: u8,
}

/// TE_USERTRACER 127
#[derive(Debug, Clone)]
pub struct TeUserTracer {
    /// `[i16; 3]`
    pub origin: Vec<i16>,
    /// `[i16; 3]`
    pub velocity: Vec<i16>,
    pub life: u8,
    pub color: u8,
    pub scale: u8,
}

/// SVC_SETPAUSE 24
#[derive(Debug, Clone)]
pub struct SvcSetPause {
    pub is_paused: i8,
}

/// SVC_SIGNONNUM 25
#[derive(Debug, Clone)]
pub struct SvcSignOnNum {
    pub sign: i8,
}

/// SVC_CENTERPRINT 26
#[derive(Debug, Clone)]
pub struct SvcCenterPrint {
    pub message: ByteVec,
}

// SVC_KILLEDMONSTER 27

// SVC_FOUNDSECRET 28

/// SVC_SPAWNSTATICSOUND 29
#[derive(Debug, Clone)]
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

// SVC_INTERMISSION 30

/// SVC_FINALE 31
#[derive(Debug, Clone)]
pub struct SvcFinale {
    pub text: ByteVec,
}

/// SVC_CDTRACK 32
#[derive(Debug, Clone)]
pub struct SvcCdTrack {
    pub track: i8,
    pub loop_track: i8,
}

/// SVC_RESTORE 33
#[derive(Debug, Clone)]
pub struct SvcRestore {
    pub save_name: ByteVec,
    pub map_count: u8,
    pub map_names: Vec<ByteVec>,
}

/// SVC_CUTSCENE 34
#[derive(Debug, Clone)]
pub struct SvcCutscene {
    pub text: ByteVec,
}

/// SVC_WEAPONANIM 35
#[derive(Debug, Clone)]
pub struct SvcWeaponAnim {
    pub sequence_number: i8,
    pub weapon_model_body_group: i8,
}

/// SVC_DECALNAME 36
#[derive(Debug, Clone)]
pub struct SvcDecalName {
    pub position_index: u8,
    pub decal_name: ByteVec,
}

/// SVC_ROOMTYPE 37
#[derive(Debug, Clone)]
pub struct SvcRoomType {
    pub room_type: u16,
}

/// SVC_ADDANGLE 38
#[derive(Debug, Clone)]
pub struct SvcAddAngle {
    pub angle_to_add: i16,
}

/// SVC_NEWUSERMSG 39
#[derive(Debug, Clone)]
pub struct SvcNewUserMsg {
    pub index: u8,
    // weird but it's for consistency
    pub size: i8,
    /// `[u8; 16]`
    pub name: ByteVec,
}

/// SVC_PACKETENTITIES 40
#[derive(Debug, Clone)]
pub struct SvcPacketEntities {
    /// `[bool; 16]`
    pub entity_count: BitVec,
    pub entity_states: Vec<EntityState>,
}
#[derive(Debug, Clone)]
pub struct EntityState {
    pub entity_index: u16,
    pub increment_entity_number: bool,
    pub is_absolute_entity_index: Option<bool>,
    /// `[bool; 11]`
    pub absolute_entity_index: Option<BitVec>,
    /// `[bool; 6]`
    pub entity_index_difference: Option<BitVec>,
    pub has_custom_delta: bool,
    pub has_baseline_index: bool,
    /// `[bool; 6]`
    pub baseline_index: Option<BitVec>,
    pub delta: Delta,
}

/// SVC_DELTAPACKETENTITIES 41
#[derive(Debug, Clone)]
pub struct SvcDeltaPacketEntities {
    /// `[bool; 16]`
    pub entity_count: BitVec,
    /// `[bool; 8]`
    pub delta_sequence: BitVec,
    pub entity_states: Vec<EntityStateDelta>,
}
#[derive(Debug, Clone)]
pub struct EntityStateDelta {
    /// `[bool; 11]` but do u16 because arithmetic.
    pub entity_index: u16,
    pub remove_entity: bool,
    pub is_absolute_entity_index: bool,
    /// `[bool; 11]`
    pub absolute_entity_index: Option<BitVec>,
    /// `[bool; 6]`
    pub entity_index_difference: Option<BitVec>,
    // Need to be optional because if remove is true then it won't have delta.
    pub has_custom_delta: Option<bool>,
    pub delta: Option<Delta>,
}

// SVC_CHOKE 42

/// SVC_RESOURCELIST 43
#[derive(Debug, Clone)]
pub struct SvcResourceList {
    /// `[bool; 12]`
    pub resource_count: BitVec,
    pub resources: Vec<Resource>,
    pub consistencies: Vec<Consistency>,
}
#[derive(Debug, Clone)]
pub struct Resource {
    /// `[bool; 4]`
    pub type_: BitVec,
    /// `&'[u8]`
    pub name: BitVec,
    /// `[bool; 12]`
    pub index: BitVec,
    /// `[bool; 24]`
    pub size: BitVec,
    /// `[bool; 3]`
    pub flags: BitVec,
    /// `[bool; 128]`
    pub md5_hash: Option<BitVec>,
    pub has_extra_info: bool,
    /// `[bool; 256]`
    pub extra_info: Option<BitVec>,
}
#[derive(Debug, Clone)]
pub struct Consistency {
    pub has_check_file_flag: bool,
    pub is_short_index: Option<bool>,
    /// `[bool; 5]`
    pub short_index: Option<BitVec>,
    /// `[bool; 10]`
    pub long_index: Option<BitVec>,
}

/// SVC_NEWMOVEVARS 44
#[derive(Debug, Clone)]
pub struct SvcNewMovevars {
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
    /// Vec3
    pub sky_color: Vec<f32>,
    /// Vec3
    pub sky_vec: Vec<f32>,
    pub sky_name: ByteVec,
}

/// SVC_RESOURCEREQUEST 45
#[derive(Debug, Clone)]
pub struct SvcResourceRequest {
    pub spawn_count: i32,
    pub unknown: Vec<u8>,
}

/// SVC_CUSTOMIZATION 46
#[derive(Debug, Clone)]
pub struct SvcCustomization {
    pub player_index: u8,
    pub type_: u8,
    pub name: ByteVec,
    pub index: u16,
    pub download_size: u32,
    pub flags: u8,
    /// `[u8; 16]`
    pub md5_hash: Option<ByteVec>,
}

/// SVC_CROSSHAIRANGLE 47
#[derive(Debug, Clone)]
pub struct SvcCrosshairAngle {
    pub pitch: i16,
    pub yaw: i16,
}

/// SVC_SOUNDFADE 48
#[derive(Debug, Clone)]
pub struct SvcSoundFade {
    pub initial_percent: u8,
    pub hold_time: u8,
    pub fade_out_time: u8,
    pub fade_in_time: u8,
}

/// SVC_FILETXFERFAILED 49
#[derive(Debug, Clone)]
pub struct SvcFileTxferFailed {
    pub file_name: ByteVec,
}

/// SVC_HLTV 50
#[derive(Debug, Clone)]
pub struct SvcHltv {
    pub mode: u8,
}

/// SVC_DIRECTOR 51
#[derive(Debug, Clone)]
pub struct SvcDirector {
    pub length: u8,
    pub flag: u8,
    pub message: ByteVec,
}

/// SVC_VOINCEINIT 52
#[derive(Debug, Clone)]
pub struct SvcVoiceInit {
    pub codec_name: ByteVec,
    pub quality: i8,
}

/// SVC_VOICEDATA 53
#[derive(Debug, Clone)]
pub struct SvcVoiceData {
    pub player_index: u8,
    pub size: u16,
    pub data: ByteVec,
}

/// SVC_SENDEXTRAINFO 54
#[derive(Debug, Clone)]
pub struct SvcSendExtraInfo {
    pub fallback_dir: ByteVec,
    pub can_cheat: u8,
}

/// SVC_TIMESCALE 55
#[derive(Debug, Clone)]
pub struct SvcTimeScale {
    pub time_scale: f32,
}

/// SVC_RESOURCELOCATION 56
#[derive(Debug, Clone)]
pub struct SvcResourceLocation {
    pub download_url: ByteVec,
}

/// SVC_SENDCVARVALUE 57
#[derive(Debug, Clone)]
pub struct SvcSendCvarValue {
    pub name: ByteVec,
}

/// SVC_SENDCVARVALUE2 58
#[derive(Debug, Clone)]
pub struct SvcSendCvarValue2 {
    pub request_id: u32,
    pub name: ByteVec,
}
