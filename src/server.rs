use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use crate::SERVER;
use color_eyre::Result;
use crossbeam::channel::RecvError;
use glam::Vec3;
use laminar::{Packet, Socket, SocketEvent};
use crate::client::SamplesWithPos;

pub fn server() -> Result<()> {
    laminar_version();
    return Ok(());
    let server_address = "74.207.246.102:8888";
    let config = Default::default();

    // Create a server object
    let mut server = uflow::server::Server::bind(server_address, config).unwrap();

    let mut client_addresses = Vec::new();
    loop {
        // Process inbound UDP frames and handle events
        for event in server.step() {
            //server.flush();
            match event {
                uflow::server::Event::Connect(client_address) => {
                    println!("[{:?}] connected", client_address);
                    client_addresses.push(client_address);
                }
                uflow::server::Event::Disconnect(client_address) => {
                    println!("[{:?}] disconnected", client_address);
                }
                uflow::server::Event::Error(client_address, err) => {
                    println!("[{:?}] error: {:?}", client_address, err);
                }
                uflow::server::Event::Receive(client_address, packet_data) => {
                    //let packet_data_utf8 = std::str::from_utf8(&packet_data).unwrap();
                    //let mut client = server.client(&client_address).unwrap().borrow_mut();
                    for (num, address) in client_addresses.iter().enumerate() {
                         //if address != &client_address {
                            server.client(address).unwrap().borrow_mut().send(
                                packet_data.clone(), num, uflow::SendMode::Unreliable
                            );
                            //server.flush();
                        //}
                    }
                    // Echo the packet reliably on channel 0
                    //client.send(packet_data, 0, uflow::SendMode::Reliable);

                    // Echo the reverse of the packet unreliably on channel 1

                    //client.send(reversed_string.as_bytes().into(), 1, uflow::SendMode::Unreliable);
                }
            }
        }

        // Flush outbound UDP frames
        //server.flush();
        //std::thread::sleep(Duration::from_nanos(2));
        server.flush();
        std::thread::sleep(std::time::Duration::from_micros(30000));
        //std::thread::sleep(std::time::Duration::from_millis(30));
    }
    Ok(())
}

fn laminar_version() {
    let mut socket = Socket::bind("127.0.0.1:8888").unwrap();
    let (mut rx, tx) = (socket.get_event_receiver(), socket.get_packet_sender());
    let _t = thread::spawn(move || socket.start_polling());
    let mut clients = HashSet::new();
    loop {
        match rx.try_recv() {
            Ok(SocketEvent::Packet(packet)) => {
                println!("recieved packet");
                clients.insert(packet.addr());
                for addr in &clients {
                    println!("sending to: {}", addr);
                    let to_send = Packet::reliable_unordered(*addr, packet.payload().to_vec());
                    tx.send(to_send).unwrap();
                }
            }
            Ok(SocketEvent::Connect(addr)) => {
                println!("connected: {}", addr);
            }
            _ => {}
        }
    }
}