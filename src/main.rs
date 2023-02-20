extern crate core;

mod window_stuff;
mod packet;
mod server;
mod client;
mod avatar;
mod things;

use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};
use color_eyre::owo_colors::AnsiColors::Default;
use laminar::{Packet, Socket, SocketEvent};
use color_eyre::Result;
use glam::{Mat4, Quat, Vec3};
use serde::{Deserialize, Serialize};
use stereokit::input::StereoKitInput;
use stereokit::Settings;
use crossbeam::channel::Sender;
use stereokit::color_named::WHITE;
use stereokit::model::Model;
use stereokit::render::RenderLayer;

const SERVER: &str = "127.0.0.1:12351";

fn server() -> Result<()> {
    let mut socket = Socket::bind(SERVER)?;
    let (mut sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    let mut players_data: HashMap<SocketAddr, PlayerData> = HashMap::new();

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(pack) => {
                    let msg = pack.payload();

                    // if msg == b"Bye!" {
                    //     break;
                    // }

                    let msg = String::from_utf8_lossy(msg);
                    let packet = PacketType::from_str(&*msg);

                    println!("Received {:?} from {:?}", packet, pack.addr());
                    match packet {
                        PacketType::PlayerPosition { socket_addr, position } => {
                            players_data.get_mut(&socket_addr).unwrap().player_position = position.clone();
                            for (addr, data) in &players_data {
                                println!("sending update to: {:?}", addr);
                                PacketType::PlayerPosition { socket_addr, position: position.clone() }.send_to_client(&mut sender, addr.clone());
                            }
                        }
                        PacketType::PlayersData(_) => {}
                        PacketType::GibData(addr) => {
                            players_data.insert(addr, PlayerData::default());
                            PacketType::PlayersData(players_data.clone()).send_to_client(&mut sender, addr);
                        }
                    }
                }
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn client(iteration: u32) -> Result<()> {
    let port_num = 12352 + iteration;
    let addr = "127.0.0.1:".to_string() + &*port_num.to_string();
    let addr = addr.as_str();
    let mut socket = Socket::bind(addr)?;
    println!("Connected on {}", addr);

    let server = SERVER.parse().unwrap();

    let addr = SocketAddr::from_str(addr).unwrap();

    println!("Type a message and press Enter to send. Send `Bye!` to quit.");

    let stdin = std::io::stdin();
    let mut s_buffer = String::new();

    let mut sk = Settings::default().init()?;

    let mut prev_position = Vec3::default();

    let mut players_data = HashMap::new();
    PacketType::GibData(addr).send_to_server(&mut socket, server);
    loop {
        socket.manual_poll(Instant::now());
        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                let packet = PacketType::from_bytes(packet.payload());
                match packet {
                    PacketType::PlayersData(data) => {
                        players_data = data;
                        break;
                    }
                    _ => panic!()
                }
            }
            _ => {}
        }
    }

    let player_model = Model::from_file(&sk, "untitled.glb", None).unwrap();
    let rx = socket.get_event_receiver();
    sk.run(|sk| {
        //s_buffer.clear();
        //stdin.read_line(&mut s_buffer).unwrap();
        //let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");

        if sk.input_head().position != prev_position.into() {
            prev_position = sk.input_head().position.into();
            let packet = PacketType::PlayerPosition { socket_addr: addr, position: PlayerPosition { position: prev_position } };
            packet.send_to_server(&mut socket, server);
        }



        socket.manual_poll(Instant::now());

        /*if line == "Bye!" {
            break;
        }*/

        match rx.try_recv() {
            Ok(SocketEvent::Packet(packet)) => {
                let packet = PacketType::from_bytes(packet.payload());
                match packet {
                    PacketType::PlayerPosition { socket_addr, position } => {
                        if players_data.contains_key(&socket_addr) {
                            players_data.get_mut(&socket_addr).unwrap().player_position = position;
                        }
                        else {
                            players_data.insert(socket_addr, PlayerData{ player_position: position });
                        }
                    }
                    PacketType::PlayersData(_) => {}
                    PacketType::GibData(_) => {}
                }
                // if packet.addr() == server {
                //     println!("Server sent: {}", String::from_utf8_lossy(packet.payload()));
                // } else {
                //     println!("Unknown sender.");
                // }
            }
            _ => {} /*println!("Silence..")*/,
        }

        for (addr, player_data) in &players_data {
            let new_position = player_data.player_position.position;
            let new_mat = Mat4::from_scale_rotation_translation(Vec3::new(0.1, 0.1, 0.1), Quat::IDENTITY, new_position);
            player_model.draw(&sk, new_mat.into(), WHITE, RenderLayer::Layer0);
        }

    }, |_| {});

    Ok(())
}

fn main() -> Result<()> {
    things::run()?;
    //client::run_client(SERVER.parse()?,SERVER.parse()?)?;
    return Ok(());
    let stdin = std::io::stdin();

    println!("Please type in `server` or `client`.");

    let mut s = String::new();
    stdin.read_line(&mut s)?;

    let server_addr = SERVER.parse()?;

    if s.starts_with('s') {
        println!("Starting server..");
        server::run_server(server_addr)
    } else {
        let s = s.replace(|x| x == '\n' || x == '\r', "");
        println!("Starting client..");
        let iteration = u32::from_str(&s).unwrap();
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), (12352 + iteration) as u16);
        client::run_client(client_addr, server_addr)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketType {
    PlayerPosition{ socket_addr: SocketAddr, position: PlayerPosition},
    PlayersData(HashMap<SocketAddr, PlayerData>),
    GibData(SocketAddr),
}

impl PacketType {
    pub fn send_to_server(self, socket: &mut Socket, server: SocketAddr) {
        let json_version = serde_json::to_string(&self).unwrap();
        socket.send(Packet::reliable_unordered(
            server,
            json_version.into_bytes(),
        )).unwrap();
    }
    pub fn send_to_client(self, sender: &mut Sender<Packet>, client: SocketAddr) {
        let json_version = serde_json::to_string(&self).unwrap();
        sender.send(Packet::reliable_unordered(
            client,
            json_version.into_bytes()
        )).unwrap();
    }
    pub fn from_str(str: &str) -> Self {
        serde_json::from_str(str).unwrap()
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let str = String::from_utf8_lossy(bytes);
        Self::from_str(&*str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerData {
    player_position: PlayerPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerPosition {
    position: Vec3,
}
