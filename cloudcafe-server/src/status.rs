use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::ops::Deref;
use std::thread;
use std::thread::JoinHandle;
use cloudcafe_common::ports::{SERVER_STATUS_PORT, ServerPlayerInfo};
use crate::{Client, Clients, SharedInfo};

pub fn setup_run_status(clients: &Clients, shared_info: &SharedInfo, server_ip: Ipv4Addr) -> JoinHandle<()> {
    let shared_info = shared_info.clone();
    let clients = clients.clone();
    thread::spawn(move || status(clients, shared_info, server_ip))
}

pub fn status(clients: Clients, shared_info: SharedInfo, server_ip: Ipv4Addr) {
    let shared_info = shared_info.clone();
    let server_address = SocketAddrV4::new(server_ip, SERVER_STATUS_PORT);
    let listener = TcpListener::bind(server_address).unwrap();
    for stream in listener.incoming() {
        shared_info.0.lock().unwrap().server_info.players = clients.iter().map(|a| {
            (a.key().clone(), a.value().clone())
        }).collect();
        let mut stream = stream.unwrap();
        let bytes = bincode::serialize(&shared_info.0.as_ref().lock().unwrap().server_info).unwrap();
        stream.write_all(bytes.as_slice()).unwrap();
    }
}

