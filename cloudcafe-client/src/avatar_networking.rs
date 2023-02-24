use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;
use laminar::{Socket, SocketEvent};

pub fn avatar_networking(server_address: Ipv4Addr) {
    thread::spawn(move || run_networking(server_address));
}

fn run_networking(server_address: Ipv4Addr) {
    let mut port_number = 0;
    let mut socket = loop {
        match Socket::bind(format!("0.0.0.0:58{}", port_number)) {
            Ok(socket) => {
                break socket;
            }
            _ => {
                port_number += 1;
            }
        }
    };
    let (mut rx, tx) = (socket.get_event_receiver(), socket.get_packet_sender());
    let _socket_thread = thread::spawn(move || socket.start_polling_with_duration(Some(Duration::from_millis(1))));
    loop {
        match rx.recv() {
            Ok(SocketEvent::Packet(packet)) => {

            }
            Ok(SocketEvent::Connect(_)) => {}
            Ok(SocketEvent::Timeout(_)) => {}
            Ok(SocketEvent::Disconnect(_)) => {}
            Err(_) => {}
        }
    }
}