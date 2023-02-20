use std::{net, thread};
use std::net::{SocketAddr, SocketAddrV4};
use std::time::Instant;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use laminar::{Packet, SocketEvent};

pub fn server(server_address: SocketAddr) -> Result<()> {
    let mut socket = laminar::Socket::bind(server_address)?;

    let (mut sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let mut client_addresses = vec![];
    let _t = thread::spawn(move || socket.start_polling());
    loop {
        if let Ok(event) = receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    println!("sending messages!");
                    for address in &client_addresses {
                        sender.send(Packet::reliable_unordered(*address, Vec::from(packet.payload()))).unwrap();
                    }
                }
                SocketEvent::Connect(connection) => {
                    println!("connecteed!: {}", connection);
                    client_addresses.push(connection);
                }
                SocketEvent::Timeout(_) => {}
                SocketEvent::Disconnect(_) => {}
            }
        }
        // for event in server.step() {
        //     match event {
        //         Event::Connect(connection) => {
        //             println!("connected: {}", connection);
        //             client_addresses.push(connection);
        //         }
        //         Event::Disconnect(dissconnect) => {
        //             println!("disconnected: {}", dissconnect);
        //         }
        //         Event::Receive(original_address, packet) => {
        //             for address in &client_addresses {
        //                 server.client(address).unwrap().borrow_mut()
        //                     .send(packet.clone(), 0, SendMode::Persistent)
        //             }
        //         }
        //         Event::Error(_, _) => {}
        //     }
        //     server.flush();
        // }
    }
    Ok(())
}