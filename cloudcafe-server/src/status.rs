use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::ops::Deref;
use std::thread;
use std::thread::JoinHandle;
use cloudcafe_common::ports::{SERVER_STATUS_PORT, ServerPlayerInfo};
use crate::{PlayerList, SharedInfo};

pub fn setup_run_status(player_list: &PlayerList, shared_info: &SharedInfo, server_ip: Ipv4Addr) -> JoinHandle<()> {
    let shared_info = shared_info.clone();
    let player_list = player_list.clone();
    thread::spawn(move || status(player_list, shared_info, server_ip))
}

pub fn status(player_list: PlayerList, shared_info: SharedInfo, server_ip: Ipv4Addr) {
    let shared_info = shared_info.clone();
    let server_address = SocketAddrV4::new(server_ip, SERVER_STATUS_PORT);
    let listener = TcpListener::bind(server_address).unwrap();
    for stream in listener.incoming() {
        shared_info.0.lock().unwrap().server_info.player_count = player_list.lock().unwrap().len() as u32;
        shared_info.0.lock().unwrap().server_info.players = player_list.lock().unwrap().iter().map(|(addr, player)| ServerPlayerInfo { addr: *addr, username: player.name.clone() }).collect();
        let mut stream = stream.unwrap();
        let bytes = bincode::serialize(&shared_info.0.as_ref().lock().unwrap().server_info).unwrap();
        stream.write_all(bytes.as_slice()).unwrap();
    }
}

