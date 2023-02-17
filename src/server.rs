use std::collections::HashMap;
use std::net::SocketAddr;
use color_eyre::Result;
use crossbeam::channel::Sender;
use laminar::Packet;
use crate::packet::{ClientPacket, PlayerInfo, ServerPacket, VPacket};

pub fn run_server(server_address: SocketAddr) -> Result<()> {
    let mut socket = laminar::Socket::bind(server_address)?;
    let (mut sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = std::thread::spawn(move || socket.start_polling());

    let mut players_info: HashMap<SocketAddr, PlayerInfo> = HashMap::new();

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                laminar::SocketEvent::Packet(pack) => {
                    let client_addr = pack.addr();
                    let packet = ClientPacket::from_bytes(pack.payload());
                    match packet {
                        ClientPacket::ConnectToServer(username) => {
                            let info = PlayerInfo::new(username);
                            players_info.insert(client_addr, info.clone());
                            let packet = ServerPacket::AddPlayer{player_addr: client_addr, player_info: info};
                            send_to_all(&players_info, &mut sender, packet);
                        }
                        ClientPacket::RequestAllPlayerInfo => {
                            ServerPacket::SendAllPlayerInfo(players_info.clone()).send_reliable_unordered(client_addr, &mut sender);
                        }
                        ClientPacket::UpdatePosition(position) => {
                            let packet = ServerPacket::UpdatePlayerPosition { player_addr: client_addr, position };
                            println!("sending update position: {:?}", packet);
                            send_to_all(&players_info, &mut sender, packet);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn send_to_all(players_info: &HashMap<SocketAddr, PlayerInfo>, sender: &mut Sender<Packet>, packet: ServerPacket) {
    for addr in players_info.keys() {
        packet.clone().send_reliable_unordered(*addr, sender);
    }
}