use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use cloudcafe_common::ports::AVATAR_PACKET_PORT;
use crate::Clients;
use color_eyre::Result;
use cloudcafe_common::laminar_helper::ConnectionHandler;

//
// fn avatar_networking(player_list: PlayerList, server_addr: Ipv4Addr) {
//     const AVATAR_NETWORKING_DURATION: Duration = Duration::from_millis(1);
//     let mut socket = Socket::bind(SocketAddrV4::new(server_addr, AVATAR_PACKET_PORT)).expect("couldn't init info socket");
//     println!("started avatar_networking socket");
//     let rx = socket.get_event_receiver();
//     let mut tx = socket.get_packet_sender();
//     loop {
//         thread::sleep(AVATAR_NETWORKING_DURATION);
//         socket.manual_poll(Instant::now());
//         for msg in rx.try_iter() {
//             match msg {
//                 SocketEvent::Packet(packet) => {
//                     for addr in player_list.lock().unwrap().keys() {
//                         if &packet.addr() == addr {
//                             continue;
//                         }
//                         let msg_packet = Packet::unreliable(*addr, packet.payload().to_vec());
//                         tx.send(msg_packet).unwrap();
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }
// }
pub fn setup_avatar_networking(clients: &Clients, server_address: Ipv4Addr) -> ConnectionHandler {
    let clients = clients.clone();
    let address = SocketAddr::V4(SocketAddrV4::new(server_address, AVATAR_PACKET_PORT));
    let socket = Socket::bind(address).unwrap();
    let (rx, tx) = (socket.get_event_receiver(), socket.get_packet_sender());
    ConnectionHandler::spawn(avatar_networking, address, None, (socket, clients, rx, tx))
}

fn avatar_networking((socket, clients, rx, tx): &mut (Socket, Clients, Receiver<SocketEvent>, Sender<Packet>)) -> Result<()> {
    socket.manual_poll(Instant::now());
    for msg in rx.try_iter() {
        match msg {
            SocketEvent::Packet(packet) => {
                for addr in clients.iter() {
                    if packet.addr() == addr.addrs.avatar_networking_addr {
                        continue;
                    }
                    let msg_packet = Packet::unreliable(addr.addrs.avatar_networking_addr, packet.payload().to_vec());
                    tx.send(msg_packet)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}