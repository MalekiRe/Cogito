mod connection_manager;
mod info_connection;
mod avatar_connection;
mod audio_connection;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use color_eyre::Result;
use dashmap::DashMap;
use glam::{Mat4, Quat, Vec3};
use log::info;
use opus::{Application, Channels};
use stereokit::lifecycle::{DisplayMode, StereoKitContext};
use stereokit::material::Material;
use stereokit::mesh::Mesh;
use stereokit::microphone::Microphone;
use stereokit::model::Model;
use stereokit::pose::Pose;
use stereokit::render::RenderLayer;
use stereokit::shader::Shader;
use stereokit::sound::{SoundInstance, SoundStream, SoundT};
use stereokit::texture::Texture;
use uuid::Uuid;
use cloudcafe_common::packet::{Client, ClientData, ServerStatus};
use stereokit_locomotion::LocomotionTracker;
use stereokit_vrm::VrmAvatar;
use crate::main_menu::{MainMenu, MainMenuMsg};
use crate::other::audio_connection::AudioPacket;
use crate::other::avatar_connection::AvatarPos;
use crate::other::connection_manager::ConnectionManager;
use crate::resources::Resources;
use crate::settings_menu::SettingsMenu;

pub type Clients = Arc<DashMap<Uuid, Client>>;
pub type Players = HashMap<Uuid, Player>;
/// this is our singleton instance of our player in this game.

pub type SinglePlayer = Rc<RefCell<Player>>;

pub fn main() -> Result<()> {
    //simple_logger::init().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let sk = stereokit::Settings::default().disable_unfocused_sleep(true).init()?;
    let connected_server_info = Arc::new(Mutex::new(None));
    let mut main_menu = MainMenu::new(&sk, connected_server_info.clone());
    let client_data = ClientData {
        name: "malek".to_string(),
        uuid: Uuid::new_v4(),
    };

    let clients = Arc::new(DashMap::new());
    let mut players: HashMap<Uuid, Player> = HashMap::new();

    let sound_stream = SoundStream::create(20.0);
    let sound = sound_stream.play_sound([0.0, 0.0, 0.0], 0.0);
    let mut player = Player {
        uuid: client_data.uuid.clone(),
        avatar: VrmAvatar::load_from_file(&sk, "Malek.vrm", &Shader::default(&sk))?,
        sound_stream,
        sound,
    };

    let mut connection_manager = ConnectionManager::new(client_data.clone(), &clients);

    let mut timer = 0;

    let mut resource = Resources::init()?;
    let mut settings_menu = SettingsMenu::from(&resource);
    let microphone = Microphone::new(1);
    microphone.start();
    println!("{}", microphone.get_name());
    let stream = microphone.get_stream().unwrap();

    let mut encoder = opus::Encoder::new(48000, Channels::Mono, Application::LowDelay).unwrap();
    let mut decoder = opus::Decoder::new(48000, Channels::Mono).unwrap();
    let mut samples = [0.0; 2880];

    let mut locomotion = LocomotionTracker::new(0.1, 1.0, 1.0);
    locomotion.stage_pose = Pose::new([-1.0, 1.0, -1.0], Quat::IDENTITY);

    let mut floor_material = Material::create(&sk, &Shader::default(&sk)).unwrap();
    let floor_texture = Texture::from_file(&sk, "grass.jpg", false, 0).unwrap();
    floor_material.set_texture(&sk, "diffuse", &floor_texture).unwrap();
    let mut floor_mesh = Mesh::gen_cube(&sk, [40.0, 1.0, 40.0], 1).unwrap();
    let mut floor_model = Model::from_mesh(&sk, &floor_mesh, &floor_material).unwrap();


    sk.run(|sk| {
        locomotion.analogue_controls(sk);
        floor_model.draw(sk, Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), Quat::IDENTITY, Vec3::new(0.0, -1.0, 0.0)).into(), stereokit::color_named::LAWN_GREEN, RenderLayer::Layer2);

        settings_menu.exist(sk);
        if let Some(msg) = main_menu.exist(sk) {
            match msg {
                MainMenuMsg::ConnectToServer(server_info) => {
                    connection_manager.connect(server_info.ip);
                    info!("connected to: {:?}", server_info);
                    let _ = connected_server_info.lock().unwrap().insert(server_info);
                }
                MainMenuMsg::Disconnect => {
                    connection_manager.disconnect();
                    if let Some(server_info) = connected_server_info.lock().unwrap().take() {
                        info!("disconnected from: {:?}", server_info);
                    }
                    players.clear();
                    clients.clear();
                }
            }
        }
        player.avatar.update_ik(sk);
        player.avatar.draw(sk, &Pose::IDENTITY);
        if let Some(connection) = &connection_manager.connection {
            let mut audio_packet = AudioPacket{
                frames: vec![],
                uuid: player.uuid.clone(),
            };
            while stream.unread_samples() >= 2880 {
                stream.read_samples(&mut samples);
                //println!("sending: {}", samples.len());
                // samples.resize(2880, 0.0);
                audio_packet.frames.push( encoder.encode_vec_float(samples.as_slice(), 2880).unwrap());
                // let mut decoder = opus::Decoder::new(48000, Channels::Mono).unwrap();
                // let mut new_samples = vec![];
                // new_samples.resize(2880, 0.0);
                // decoder.decode_float(encoded.as_slice(), &mut new_samples, false).unwrap();
                // println!("encoded len: {}", encoded.len());
                // sound_str.write_samples(&new_samples);
            }
            if !audio_packet.frames.is_empty() {
                connection.audio_this_tx.send(audio_packet).unwrap();
            }
            for other_audio in connection.audio_other_rx.try_iter() {
                if let Some(p) = players.get_mut(&other_audio.uuid) {
                    println!("got audio");
                    for audio in other_audio.frames {
                        decoder.decode_float(&audio, &mut samples, false).unwrap();
                        p.sound_stream.write_samples(&samples);
                    }
                    p.sound.set_position(p.avatar.node_get_pose_model(p.avatar.skeleton.torso.hips).position);
                }
            }

            connection.this_avatar_tx.send(AvatarPos {
                uuid: client_data.uuid,
                pos: player.avatar.get_nodes_and_poses(sk),
            }).unwrap();

            for other_avatar_pos in connection.other_avatar_rx.try_iter() {
                if let Some(other) = players.get_mut(&other_avatar_pos.uuid) {
                    other.avatar.set_nodes_and_poses(other_avatar_pos.pos);
                }
            }
            for p in players.values() {
                p.avatar.draw(sk, &Pose::IDENTITY);
            }
            for server_message in connection.server_status_rx.try_iter() {
                match server_message {
                    ServerStatus::ServerInfo(server_info) => {
                        let _ = connected_server_info.lock().unwrap().insert(server_info.clone());
                        clients.clear();
                        players.clear();
                        for (uuid, c) in server_info.players {
                            if uuid == client_data.uuid {
                                continue;
                            }
                            add_user(sk, c, &clients, &mut players);
                        }
                    }
                    ServerStatus::Kick => {
                        todo!()
                    }
                    ServerStatus::ClientDisconnected(uuid) => {
                        remove_user(uuid, &clients, &mut players);
                        connected_server_info.lock().unwrap().as_mut().unwrap().players.remove(&uuid);
                    }
                    ServerStatus::ClientConnected(client) => {
                        connected_server_info.lock().unwrap().as_mut().unwrap().players.insert(client.data.uuid.clone(), client.clone());
                        add_user(sk, client, &clients, &mut players);
                    }
                    ServerStatus::Heartbeat => {}
                }
            }
        }
    }, |_| {});
    Ok(())
}

pub fn remove_user(uuid: Uuid, clients: &Clients, players: &mut Players) {
    if let Some(client) = clients.remove(&uuid) {
        info!("removing user: {:?}", client.1);
    }
    players.remove(&uuid);

}
pub fn add_user(sk: &impl StereoKitContext, client: Client, clients: &Clients, players: &mut Players) {
    info!("adding user: {:?}", client);
    let stream = SoundStream::create(20.0);
    players.insert(client.data.uuid, Player {
        uuid: client.data.uuid,
        avatar: VrmAvatar::load_from_file(sk, "Malek.vrm", &Shader::default(sk)).unwrap(),
        sound: stream.play_sound([0.0, 0.0, 0.0], 3.0),
        sound_stream: stream,
    });
    clients.insert(client.data.uuid, client);
}

// client is stuff for over the network direct communication and information, player is stuff that is local to each instance and bits and pieces may or may not be synced over the network
pub struct Player {
    uuid: Uuid,
    avatar: VrmAvatar,
    sound_stream: SoundStream,
    sound: SoundInstance,
}