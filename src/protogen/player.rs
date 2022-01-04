#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInGameAction {
    #[prost(int32, tag="1")]
    pub target_player_id: i32,
    #[prost(string, tag="2")]
    pub action_type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameEvent {
    #[prost(int32, tag="1")]
    pub player_id: i32,
    #[prost(int32, tag="2")]
    pub target_player_id: i32,
    #[prost(string, tag="3")]
    pub action_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
}
