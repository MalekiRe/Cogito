use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread;
use std::time::Instant;
use color_eyre::Result;
use crossbeam::channel::TryRecvError;
use glam::Vec3;
use laminar::SocketEvent;
use stereokit::color_named::WHITE;
use stereokit::input::{Handed, StereoKitInput};
use stereokit::render::RenderLayer;
use crate::packet::{ClientPacket, ServerPacket, VPacket};

pub fn run_client(client_address: SocketAddr, server_address: SocketAddr) -> Result<()> {
    let mut socket = laminar::Socket::bind(client_address)?;
    let (mut sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

    let mut sk = stereokit::Settings::default().init()?;

    let mut prev_position = Vec3::default();
    let name = anarchist_readable_name_generator_lib::readable_name();
    ClientPacket::ConnectToServer(name).send_reliable_unordered(server_address, &mut sender);
    ClientPacket::RequestAllPlayerInfo.send_reliable_unordered(server_address, &mut sender);
    let mut players_info = HashMap::new();
    loop {
        socket.manual_poll(Instant::now());
        match receiver.try_recv() {
            Ok(SocketEvent::Packet(pack)) => {
                let packet = ServerPacket::from_bytes(pack.payload());
                match packet {
                    ServerPacket::SendAllPlayerInfo(info) => {
                        players_info = info;
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    let _thread = thread::spawn(move || socket.start_polling());
    let player_model = stereokit::model::Model::from_file(&sk, "Malek.vrm", None).unwrap();
    let node = player_model.node_find("J_Bip_R_Hand").unwrap();
    sk.run(|sk| {

        if sk.input_head().position != prev_position.into() {
            prev_position = sk.input_head().position.into();
            ClientPacket::UpdatePosition(prev_position).send_reliable_unordered(server_address, &mut sender);
        }

        player_model.node_set_transform_model(node, sk.input_hand(Handed::Right).wrist.as_matrix());

        //socket.manual_poll(Instant::now());


        match receiver.try_recv() {
            Ok(SocketEvent::Packet(pack)) => {
                let packet = ServerPacket::from_bytes(pack.payload());
                match packet {
                    ServerPacket::UpdatePlayerPosition { player_addr, position } => {
                        println!("receiving updated player: {}, position: {}", player_addr, position);
                        players_info.get_mut(&player_addr).unwrap().position = position;
                    }
                    ServerPacket::AddPlayer { player_addr, player_info } => {
                        players_info.insert(player_addr, player_info);
                    }
                    _ => {}
                }
            }
            _ => {} /*println!("Silence..")*/,
        }

        for (addr, player_info) in &players_info {
            let mut new_position = player_info.position;
            if addr == &client_address {
                new_position = sk.input_head().position.into();
            }
            let new_mat = glam::Mat4::from_scale_rotation_translation(Vec3::new(0.1, 0.1, 0.1), glam::Quat::IDENTITY, new_position);
            player_model.draw(&sk, new_mat.into(), WHITE, RenderLayer::Layer0);
        }

    }, |_| {});

    Ok(())
}