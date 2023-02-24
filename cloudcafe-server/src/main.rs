use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use cloudcafe_common::ports::{Map, ServerInfo};
use crate::info::run_setup_info;
use crate::status::setup_run_status;

mod status;
mod info;

const SERVER_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
fn main() {
    let server_info = ServerInfo {
        name: "malek's server".to_string(),
        ip: SERVER_IP,
        player_count: 0,
        players: vec![],
        map: Map::FlatGrass,
    };
    let player_list: PlayerList = Arc::new(Mutex::new(HashMap::new()));
    let shared_info = SharedInfo::new(server_info);
    let status_thread = setup_run_status(&player_list, &shared_info, SERVER_IP);
    let info_thread = run_setup_info(&player_list, &shared_info, SERVER_IP);
    info_thread.join().unwrap();
    status_thread.join().unwrap();
}

pub type PlayerList = Arc<Mutex<HashMap<SocketAddr, Player>>>;

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
}

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