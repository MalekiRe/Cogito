use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use crate::ports::ServerInfo;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ClientStatus {
    Connect(ClientInfo),
    ClientInfo(ClientInfo),
    Disconnect,
    Heartbeat,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ServerStatus {
    ServerInfo(ServerInfo),
    Kick,
    ClientDisconnected(SocketAddr),
    ClientConnected((SocketAddr, ClientInfo)),
    Heartbeat,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub name: String,
}