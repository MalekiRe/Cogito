use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::ports::ServerInfo;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ClientStatus {
    Connect(Client),
    ClientInfo(Client),
    Disconnect(Uuid),
    Heartbeat,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ServerStatus {
    ServerInfo(ServerInfo),
    Kick,
    ClientDisconnected(Uuid),
    ClientConnected(Client),
    Heartbeat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Client {
    pub data: ClientData,
    pub addrs: ClientAddresses,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientData {
    pub name: String,
    pub uuid: Uuid,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientAddresses {
    pub info_addr: SocketAddr,
    pub avatar_networking_addr: SocketAddr,
    pub audio_addr: SocketAddr,
}