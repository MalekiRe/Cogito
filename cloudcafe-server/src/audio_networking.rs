use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Instant;
use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use cloudcafe_common::laminar_helper::ConnectionHandler;
use cloudcafe_common::ports::VOICE_COMM_PORT;
use crate::Clients;
use color_eyre::Result;

pub fn setup_audio_networking(clients: &Clients, server_address: Ipv4Addr) -> ConnectionHandler {
    let clients = clients.clone();
    let address = SocketAddr::V4(SocketAddrV4::new(server_address, VOICE_COMM_PORT));
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
                    if packet.addr() == addr.addrs.audio_addr {
                        continue;
                    }
                    let msg_packet = Packet::reliable_ordered(addr.addrs.audio_addr, packet.payload().to_vec(), Some(2));
                    tx.send(msg_packet)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}