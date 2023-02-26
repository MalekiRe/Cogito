use std::net::SocketAddr;
use std::time::Instant;
use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use stereokit::model::NodeId;
use stereokit::pose::Pose;
use uuid::Uuid;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AvatarPos {
    pub(crate) uuid: Uuid,
    pub(crate) pos: Vec<(NodeId, Pose)>
}

pub fn avatar_connection((socket, other_avatar_tx, rx, tx, user_avatar_rx, server_addr): &mut (Socket, Sender<AvatarPos>, Receiver<SocketEvent>, Sender<Packet>, Receiver<AvatarPos>, SocketAddr)) -> Result<()> {
    socket.manual_poll(Instant::now());
    for avatar in user_avatar_rx.try_iter() {
        tx.send(Packet::unreliable(server_addr.clone(), bincode::serialize(&avatar)?))?;
    }
    for avatar in rx.try_iter() {
        match avatar {
            SocketEvent::Packet(packet) => {
                other_avatar_tx.send(bincode::deserialize(packet.payload())?)?;
            }
            _ => {}
        }
    }
    Ok(())
}