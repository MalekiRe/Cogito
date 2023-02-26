use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use laminar::{Packet, Socket, SocketEvent};
use cloudcafe_common::laminar_helper::ConnectionTrigger;
use color_eyre::Result;
use crossbeam::channel::{Sender, TryRecvError};
use uuid::Uuid;
use cloudcafe_common::packet::{Client, ClientStatus, ServerStatus};

pub fn info_connection(connection_trigger: ConnectionTrigger, sleep_duration: Option<Duration>, client: Client, mut socket: Socket, server_address: SocketAddr, server_status_tx: Sender<ServerStatus>) -> Result<()> {
    let mut rx = socket.get_event_receiver();
    let mut tx = socket.get_packet_sender();
    let heartbeat_packet = Packet::unreliable(server_address, bincode::serialize(&ClientStatus::Heartbeat).expect("couldn't serialize the heartbeat packet"));

    tx.send(Packet::reliable_unordered(server_address, bincode::serialize(&ClientStatus::Connect(client.clone())).unwrap())).unwrap();

    loop {
        socket.manual_poll(Instant::now());
        if let Some(sleep_duration) = sleep_duration {
            thread::sleep(sleep_duration);
        }
        tx.send(heartbeat_packet.clone()).expect("couldn't send heartbeat packet");
        if connection_trigger.disconnected() {
            tx.send(Packet::reliable_unordered(server_address, bincode::serialize(&ClientStatus::Disconnect(client.data.uuid.clone())).unwrap())).unwrap();
            for _ in 0..4 {
                socket.manual_poll(Instant::now());
                thread::sleep(Duration::from_millis(1));
            }
            return Ok(());
        }
        match rx.try_recv() {
            Ok(SocketEvent::Packet(packet)) => {
                let status: ServerStatus = bincode::deserialize(packet.payload()).unwrap();
                server_status_tx.send(status).unwrap();
            }
            otherwise => {

            }
        }
    }
    Ok(())
}