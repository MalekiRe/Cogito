use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::{Duration, Instant};
use laminar::{Packet, Socket, SocketEvent};
use cloudcafe_common::laminar_helper::ConnectionHandler;
use cloudcafe_common::ports::SEND_BACK_PORT;
use color_eyre::Result;

pub fn setup_send_back_address(server_address: Ipv4Addr) -> ConnectionHandler {
    let address = SocketAddr::V4(SocketAddrV4::new(server_address, SEND_BACK_PORT));
    let socket = Socket::bind(address).unwrap();

    ConnectionHandler::spawn(send_back_address, address, Some(Duration::from_millis(1)), (socket))
}

pub fn send_back_address((socket): &mut (Socket)) -> Result<()> {
    let socket: &mut Socket = socket;
    socket.manual_poll(Instant::now());
    for msg in socket.get_event_receiver().try_iter() {
        match msg {
            SocketEvent::Packet(packet) => {
                socket.send(Packet::reliable_unordered(packet.addr(), bincode::serialize(&packet.addr()).unwrap())).unwrap();
            }
            _ => {}
        }
    }
    Ok(())
}