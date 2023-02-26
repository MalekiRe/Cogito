use std::collections::HashMap;
use std::io::stdin;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cloudcafe_common::packet::Client;
use cloudcafe_common::ports::{Map, ServerInfo};
use crate::avatar_networking::setup_avatar_networking;
use crate::info::setup_info_connection;
use crate::status::setup_run_status;

mod status;
mod info;
mod avatar_networking;

const SERVER_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
fn main() {
    simple_logger::init().unwrap();
    let server_info = ServerInfo {
        name: "malek's server".to_string(),
        ip: SERVER_IP,
        players: Default::default(),
        map: Map::FlatGrass,
    };
    let clients: Clients = Arc::new(DashMap::new());
    let shared_info = SharedInfo::new(server_info);
    let status_thread = setup_run_status(&clients, &shared_info, SERVER_IP);
    let info_connection = setup_info_connection(&clients, &shared_info, SERVER_IP);
    let avatar_networking = setup_avatar_networking(&clients, SERVER_IP);

    let stdin = stdin();
    stdin.read_line(&mut String::new()).unwrap();
    info_connection.disconnect().unwrap();
    avatar_networking.disconnect().unwrap();
}
//
// pub type PlayerList = Arc<Mutex<PlayerListInner>>;
//
// #[derive(Clone)]
// pub struct PlayerListInner {
//     pub info_list: HashMap<SocketAddr, Player>,
//     pub networking_list: Arc<Mutex<Vec<SocketAddr>>>,
// }
//
// impl PlayerListInner {
//     pub fn new() -> Self {
//         Self {
//             info_list: Arc::new(Mutex::new(Default::default())),
//             networking_list: Arc::new(Mutex::new(vec![])),
//         }
//     }
//     pub fn remove(&mut self, addr: SocketAddr) {
//         self.info_list.lock().unwrap().remove(&addr);
//         self.networking_list.lock().unwrap().
//     }
// }

pub type Clients = Arc<DashMap<Uuid, Client>>;

#[derive(Debug)]
pub struct InnerSharedInfo {
    server_info: ServerInfo
}


#[derive(Clone, Debug)]
pub struct SharedInfo(pub Arc<Mutex<InnerSharedInfo>>);
impl SharedInfo {
    pub fn new(server_info: ServerInfo) -> Self {
        Self (Arc::new(Mutex::new(InnerSharedInfo { server_info })))
    }
}


