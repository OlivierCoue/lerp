#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(float, tag="1")]
    pub x: f32,
    #[prost(float, tag="2")]
    pub y: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpPolygon {
    #[prost(message, repeated, tag="1")]
    pub points: ::std::vec::Vec<Point>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntPoint {
    #[prost(int32, tag="1")]
    pub x: i32,
    #[prost(int32, tag="2")]
    pub y: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticAsset {
    #[prost(enumeration="StaticAssetType", tag="1")]
    pub r#type: i32,
    #[prost(enumeration="Orientation", tag="2")]
    pub orientation: i32,
    #[prost(message, optional, tag="3")]
    pub coordinate: ::std::option::Option<IntPoint>,
    #[prost(int32, tag="4")]
    pub layer: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tile {
    #[prost(enumeration="TileType", tag="1")]
    pub tiletype: i32,
    #[prost(bool, tag="2")]
    pub walkable: bool,
    #[prost(uint32, tag="3")]
    pub layer: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TileRow {
    #[prost(message, repeated, tag="1")]
    pub tiles: ::std::vec::Vec<Tile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TileGrid {
    #[prost(message, repeated, tag="1")]
    pub grid: ::std::vec::Vec<TileRow>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameEntityBaseType {
    Character = 0,
    Projectile = 1,
    Enemy = 2,
    Wall = 3,
    MeleeAttack = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TileType {
    Floor = 0,
    Water = 1,
    Forest = 2,
    Rock = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StaticAssetType {
    Barell = 0,
    Fence = 1,
    Tree = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Orientation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UdpSpell {
    SpellProjectile = 0,
    SpellFrozenOrb = 1,
    SpellMeleeAttack = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpMsgDownGameEntityUpdate {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(enumeration="GameEntityBaseType", tag="2")]
    pub object_type: i32,
    #[prost(message, optional, tag="3")]
    pub location_current: ::std::option::Option<Point>,
    #[prost(message, repeated, tag="4")]
    pub location_target_queue: ::std::vec::Vec<Point>,
    #[prost(float, tag="5")]
    pub velocity_speed: f32,
    #[prost(message, optional, tag="6")]
    pub collider_dmg_in_rect: ::std::option::Option<Point>,
    #[prost(message, optional, tag="7")]
    pub collider_mvt: ::std::option::Option<UdpColliderMvt>,
    #[prost(uint32, tag="8")]
    pub health_current: u32,
    #[prost(bool, tag="9")]
    pub is_self: bool,
    #[prost(message, optional, tag="10")]
    pub cast: ::std::option::Option<UdpCast>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpColliderMvt {
    #[prost(bool, tag="1")]
    pub reversed: bool,
    #[prost(message, optional, tag="2")]
    pub rect: ::std::option::Option<Point>,
    #[prost(message, repeated, tag="3")]
    pub poly: ::std::vec::Vec<Point>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpCast {
    #[prost(enumeration="UdpSpell", tag="1")]
    pub spell: i32,
    #[prost(message, optional, tag="2")]
    pub target: ::std::option::Option<Point>,
    #[prost(uint32, tag="3")]
    pub duration: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpMsgDownGameEntityRemoved {
    #[prost(uint32, tag="1")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpMsgDownUserConnectFailed {
    #[prost(string, tag="1")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpMsgDownUserCreateWorldInstanceSuccess {
    #[prost(string, tag="1")]
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpMsgDownAreaInit {
    #[prost(float, tag="1")]
    pub width: f32,
    #[prost(float, tag="2")]
    pub height: f32,
    #[prost(message, repeated, tag="3")]
    pub oob_polygons: ::std::vec::Vec<UdpPolygon>,
    #[prost(enumeration="TileType", tag="4")]
    pub oob_tile_type: i32,
    #[prost(message, optional, tag="5")]
    pub area_grid: ::std::option::Option<TileGrid>,
    #[prost(message, repeated, tag="6")]
    pub static_assets: ::std::vec::Vec<StaticAsset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpMsgDown {
    #[prost(enumeration="UdpMsgDownType", tag="1")]
    pub r#type: i32,
    #[prost(message, optional, tag="2")]
    pub game_entity_update: ::std::option::Option<UdpMsgDownGameEntityUpdate>,
    #[prost(message, optional, tag="3")]
    pub game_entity_removed: ::std::option::Option<UdpMsgDownGameEntityRemoved>,
    #[prost(message, optional, tag="4")]
    pub user_connect_failed: ::std::option::Option<UdpMsgDownUserConnectFailed>,
    #[prost(message, optional, tag="5")]
    pub user_create_world_instance_success: ::std::option::Option<UdpMsgDownUserCreateWorldInstanceSuccess>,
    #[prost(message, optional, tag="6")]
    pub area_init: ::std::option::Option<UdpMsgDownAreaInit>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpMsgDownWrapper {
    #[prost(uint64, tag="1")]
    pub server_time: u64,
    #[prost(message, repeated, tag="2")]
    pub messages: ::std::vec::Vec<UdpMsgDown>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UdpMsgDownType {
    GameEntityUpdate = 0,
    GameEntityRemoved = 1,
    UserConnectSuccess = 3,
    UserConnectFailed = 4,
    UserDisconnectSuccess = 5,
    UserCreateWordlInstanceSuccess = 6,
    UserLeaveWorldInstanceSuccess = 7,
    AreaInit = 8,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpUserJoinWorldInstance {
    #[prost(string, tag="1")]
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUp {
    #[prost(enumeration="MsgUpType", tag="1")]
    pub r#type: i32,
    #[prost(message, optional, tag="2")]
    pub player_move: ::std::option::Option<Point>,
    #[prost(message, optional, tag="3")]
    pub player_teleport: ::std::option::Option<Point>,
    #[prost(message, optional, tag="4")]
    pub player_throw_projectile: ::std::option::Option<Point>,
    #[prost(message, optional, tag="5")]
    pub player_throw_frozen_orb: ::std::option::Option<Point>,
    #[prost(string, tag="6")]
    pub user_connect_username: std::string::String,
    #[prost(message, optional, tag="7")]
    pub user_join_world_instance: ::std::option::Option<MsgUpUserJoinWorldInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpWrapper {
    #[prost(message, repeated, tag="1")]
    pub messages: ::std::vec::Vec<MsgUp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpHandshake {
    #[prost(string, tag="1")]
    pub user_uuid: std::string::String,
    #[prost(string, tag="2")]
    pub signed_message: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgUpType {
    GamePause = 0,
    PlayerInit = 1,
    PlayerMove = 2,
    PlayerTeleport = 3,
    PlayerThrowProjectile = 4,
    PlayerThrowFrozenOrb = 5,
    PlayerMeleeAttack = 6,
    SettingsToggleEnemies = 7,
    UserConnect = 8,
    UserDisconnect = 9,
    UserCreateWorldInstance = 10,
    UserJoinWoldInstance = 11,
    UserLeaveWorldInstance = 12,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpError {
    #[prost(string, tag="1")]
    pub message: std::string::String,
}
////// Register

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRegisterInput {
    #[prost(string, tag="1")]
    pub username: std::string::String,
    #[prost(string, tag="2")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRegisterResponse {
}
////// Login

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpLoginInput {
    #[prost(string, tag="1")]
    pub username: std::string::String,
    #[prost(string, tag="2")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpLoginResponse {
    #[prost(string, tag="1")]
    pub uuid: std::string::String,
    #[prost(string, tag="2")]
    pub username: std::string::String,
    #[prost(string, tag="3")]
    pub auth_token: std::string::String,
    #[prost(string, tag="4")]
    pub game_server_aes_key: std::string::String,
    #[prost(string, tag="5")]
    pub game_server_aes_nonce: std::string::String,
    #[prost(string, tag="6")]
    pub game_server_handshake_challenge: std::string::String,
}
////// Logout

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpLogoutResponse {
}
////// User get current

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpUserGetCurrentResponse {
    #[prost(string, tag="1")]
    pub uuid: std::string::String,
    #[prost(string, tag="2")]
    pub username: std::string::String,
}
