use std::net::SocketAddr;
use std::time::Instant;
use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use serde::{Deserialize, Serialize};
use stereokit::values::MVec3;
use uuid::Uuid;
use color_eyre::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct AudioPacket {
    pub frames: Vec<Vec<u8>>,
    pub uuid: Uuid,
}

pub fn audio_connection((socket, audio_net_rx, audio_net_tx, other_audio_tx, this_audio_rx, server_address): &mut (Socket, Receiver<SocketEvent>, Sender<Packet>, Sender<AudioPacket>, Receiver<AudioPacket>, SocketAddr)) -> Result<()> {
    socket.manual_poll(Instant::now());
    for msg in audio_net_rx.try_iter() {
        match msg {
            SocketEvent::Packet(packet) => {
                let audio_packet: AudioPacket = bincode::deserialize(packet.payload())?;
                other_audio_tx.send(audio_packet)?;
            }
            _ => {}
        }
    }
    for audio_packet in this_audio_rx.try_iter() {
        let packet = Packet::reliable_ordered(server_address.clone(), bincode::serialize(&audio_packet)?, Some(1));
        audio_net_tx.send(packet)?;
    }
    Ok(())
}