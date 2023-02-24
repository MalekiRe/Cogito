use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use crossbeam::channel::Sender;
use laminar::{Packet, Socket, SocketEvent};
use cloudcafe_common::packet::{ClientInfo, ClientStatus, ServerStatus};
use cloudcafe_common::ports::{CLIENT_INFO_PORT, ServerInfo, ServerPlayerInfo};
use crate::{Player, PlayerList, SharedInfo};

pub fn run_setup_info(player_list: &PlayerList, shared_info: &SharedInfo, server_addr: Ipv4Addr) -> JoinHandle<()> {
    let player_list = player_list.clone();
    let shared_info = shared_info.clone();
    thread::spawn(move || info(player_list, shared_info, server_addr))
}

fn info(player_list: PlayerList, shared_info: SharedInfo, server_addr: Ipv4Addr) {
    const INFO_DURATION: Duration = Duration::from_millis(1);
    let mut socket = Socket::bind(SocketAddrV4::new(server_addr, CLIENT_INFO_PORT)).expect("couldn't init info socket");
    println!("started info socket");
    let rx = socket.get_event_receiver();
    let mut tx = socket.get_packet_sender();
    loop {
        thread::sleep(INFO_DURATION);
        socket.manual_poll(Instant::now());
        for msg in rx.try_iter() {
            match msg {
                SocketEvent::Packet(packet) => {
                    let client_info: ClientStatus = bincode::deserialize(packet.payload()).expect("couldn't deserialize client info packet");
                    match client_info {
                        ClientStatus::Connect(client_info) => {
                                let player = Player {
                                    name: client_info.name.clone(),
                                };
                                println!("player: {:?} addr: {} connected", player, packet.addr());
                                send_connect_message(&player_list, (packet.addr(), client_info), &mut tx);
                                send_server_info(packet.addr(), &player_list, &shared_info, &mut tx);
                                player_list.lock().unwrap().insert(packet.addr(), player);
                        }
                        ClientStatus::ClientInfo(_) => {}
                        ClientStatus::Disconnect => {
                            let addr = packet.addr();
                            let player = player_list.lock().unwrap().remove(&addr);
                            if let Some(player) = player {
                                println!("player: {:?} addr: {} disconnected", player, addr);
                                send_disconnect_message(&player_list, addr, &mut tx);
                            }
                        }
                        ClientStatus::Heartbeat => {}
                    }
                }
                SocketEvent::Connect(_) => {}
                SocketEvent::Timeout(addr) |
                SocketEvent::Disconnect(addr) => {
                    let player = player_list.lock().unwrap().remove(&addr);
                    if let Some(player) = player {
                        send_disconnect_message(&player_list, addr, &mut tx);
                        println!("player: {:?} addr: {} disconnected without saying goodbye or timed out ( or rejoined and left in rapid succession )", player, addr);
                    }
                }
            }
        }
    }
}

fn send_connect_message(player_list: &PlayerList, info: (SocketAddr, ClientInfo), tx: &mut Sender<Packet>) {
    let msg = ServerStatus::ClientConnected(info);
    let data = bincode::serialize(&msg).unwrap();
    for player in player_list.lock().unwrap().keys() {
        let packet = Packet::reliable_unordered(*player, data.clone());
        tx.send(packet).unwrap();
    }
}
fn send_server_info(client_addr: SocketAddr, player_list: &PlayerList, shared_info: &SharedInfo, tx: &mut Sender<Packet>) {
    shared_info.0.lock().unwrap().server_info.player_count = player_list.lock().unwrap().len() as u32;
    shared_info.0.lock().unwrap().server_info.players = player_list.lock().unwrap().iter().map(|(addr, player)| ServerPlayerInfo { addr: *addr, username: player.name.clone() }).collect();
    let msg = ServerStatus::ServerInfo(shared_info.0.lock().unwrap().server_info.clone());
    let data = bincode::serialize(&msg).unwrap();
    let packet = Packet::reliable_unordered(client_addr, data);
    tx.send(packet).unwrap();
}
fn send_disconnect_message(player_list: &PlayerList, disconnect_addr: SocketAddr, tx: &mut Sender<Packet>) {
    let msg = ServerStatus::ClientDisconnected(disconnect_addr);
    let data = bincode::serialize(&msg).unwrap();
    for player in player_list.lock().unwrap().keys() {
        let packet = Packet::reliable_unordered(*player, data.clone());
        tx.send(packet).unwrap();
    }
}