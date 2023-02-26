use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::packet::Client;

pub const SERVER_STATUS_PORT: u16 = 4800;
pub const AVATAR_PACKET_PORT: u16 = 4801;
pub const VOICE_COMM_PORT: u16 = 4082;
pub const CLIENT_INFO_PORT: u16 = 4083;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerInfo {
    pub name: String,
    pub ip: Ipv4Addr,
    pub players: HashMap<Uuid, Client>,
    pub map: Map,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerPlayerInfo {
    pub addr: SocketAddr,
    pub username: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Map {
    FlatGrass
}