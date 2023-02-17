use std::collections::HashMap;
use std::net::SocketAddr;
use glam::Vec3;
use laminar::Packet;
use serde::{Deserialize, Serialize};

pub trait VPacket: Serialize + for<'a> Deserialize<'a> + Sized {
    fn send_reliable_ordered(&self, addr: SocketAddr, sender: &mut crossbeam::channel::Sender<Packet>) {
        let bytes = bincode::serialize(self).unwrap();
        sender.send(Packet::reliable_ordered(addr, bytes, None)).unwrap();
    }
    fn send_reliable_unordered(&self, addr: SocketAddr, sender: &mut crossbeam::channel::Sender<Packet>) {
        let bytes = bincode::serialize(self).unwrap();
        sender.send(Packet::reliable_unordered(addr, bytes)).unwrap();
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

impl VPacket for ServerPacket{}
impl VPacket for ClientPacket{}

/// server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerPacket {
    UpdatePlayerPosition{ player_addr: SocketAddr, position: Vec3},
    AddPlayer{player_addr: SocketAddr, player_info: PlayerInfo},
    SendAllPlayerInfo(HashMap<SocketAddr, PlayerInfo>),
}

/// client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientPacket {
    ConnectToServer(String),
    RequestAllPlayerInfo,
    UpdatePosition(Vec3),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub username: String,
    pub position: Vec3,
}

impl PlayerInfo {
    pub fn new(username: String) -> Self {
        Self {
            username,
            position: Default::default(),
        }
    }
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            username: "default_username".to_string(),
            position: Default::default(),
        }
    }
}