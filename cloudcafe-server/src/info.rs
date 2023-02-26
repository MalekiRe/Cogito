use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use color_eyre::owo_colors::OwoColorize;
use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use cloudcafe_common::laminar_helper::ConnectionHandler;
use cloudcafe_common::ports::{CLIENT_INFO_PORT, ServerInfo, ServerPlayerInfo};
use crate::{Client, Clients, SharedInfo};
use log::{info, warn};
use color_eyre::Result;
use uuid::Uuid;
use cloudcafe_common::packet::{ClientStatus, ServerStatus};

pub fn setup_info_connection(clients: &Clients, shared_info: &SharedInfo, server_addr: Ipv4Addr) -> ConnectionHandler {
    let clients = clients.clone();
    let shared_info = shared_info.clone();
    let address = SocketAddrV4::new(server_addr, CLIENT_INFO_PORT);
    let mut socket = Socket::bind(address).expect("couldn't init info socket");
    info!("started info socket");
    let rx = socket.get_event_receiver();
    let mut tx = socket.get_packet_sender();
    ConnectionHandler::spawn(run_info_connection, SocketAddr::V4(address), Some(Duration::from_millis(5)), (clients, shared_info, rx, tx, socket))
}

fn run_info_connection((clients, shared_info, rx, tx, socket): &mut (Clients, SharedInfo, Receiver<SocketEvent>, Sender<Packet>, Socket)) -> Result<()> {
    socket.manual_poll(Instant::now());
    for msg in rx.try_iter() {
        match msg {
            SocketEvent::Packet(packet) => {
                if packet.payload().len() == 0 {
                    tx.send(Packet::reliable_unordered(packet.addr(), bincode::serialize(&packet.addr()).unwrap())).unwrap();
                    continue;
                }
                let client_info: ClientStatus = bincode::deserialize(packet.payload()).expect("couldn't deserialize client info packet");
                match client_info {
                    ClientStatus::Connect(client) => {
                        info!("client connected: {:?}", client);
                        clients.insert(client.data.uuid.clone(), client.clone());
                        send_connect_message(&clients, client.clone(), tx);
                        send_server_info(client.addrs.info_addr, &clients, &shared_info, tx);
                    }
                    ClientStatus::ClientInfo(_) => {}
                    ClientStatus::Disconnect(uuid) => {
                        if let Some(client) = clients.remove(&uuid) {
                            info!("client disconnected gracefully: {:?}", client.1);
                            send_disconnect_message(&clients, uuid, tx);
                        }
                    }
                    ClientStatus::Heartbeat => {}
                }
            }
            SocketEvent::Connect(_) => {}
            SocketEvent::Timeout(addr)  |
            SocketEvent::Disconnect(addr) => {
                let mut possible_addr = None;
                for thing in clients.iter() {
                    if thing.addrs.info_addr == addr {
                        possible_addr = Some(thing.data.uuid);
                    }
                }
                if let Some(address) = possible_addr {
                    let client = clients.remove(&address).expect("address was previously found and shouldn't have been removed before this one");
                    warn!("client timed out {{ or rejoined and left rapidly }}: {:?}", client.1);
                }
            }
        }
    }
    Ok(())
}

fn send_connect_message(clients: &Clients, client: Client, tx: &mut Sender<Packet>) {
    let msg = ServerStatus::ClientConnected(client);
    let data = bincode::serialize(&msg).unwrap();
    for client_to_send in clients.iter() {
        let packet = Packet::reliable_unordered(client_to_send.addrs.info_addr.clone(), data.clone());
        tx.send(packet).unwrap();
    }
}
fn send_server_info(client_addr: SocketAddr, clients: &Clients, shared_info: &SharedInfo, tx: &mut Sender<Packet>) {
    shared_info.0.lock().unwrap().server_info.players = clients.iter().map(|c| (c.key().clone(), c.value().clone())).collect();
    let msg = ServerStatus::ServerInfo(shared_info.0.lock().unwrap().server_info.clone());
    let data = bincode::serialize(&msg).unwrap();
    let packet = Packet::reliable_unordered(client_addr, data);
    tx.send(packet).unwrap();
}
fn send_disconnect_message(clients: &Clients, disconnect_uuid: Uuid, tx: &mut Sender<Packet>) {
    let msg = ServerStatus::ClientDisconnected(disconnect_uuid);
    let data = bincode::serialize(&msg).unwrap();
    for client in clients.iter() {
        let packet = Packet::reliable_unordered(client.addrs.info_addr.clone(), data.clone());
        tx.send(packet).unwrap();
    }
}