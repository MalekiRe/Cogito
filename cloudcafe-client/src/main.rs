// use std::collections::HashMap;
// use std::net::SocketAddr;
// use std::sync::{Arc, Mutex};
// use color_eyre::owo_colors::OwoColorize;
// use dashmap::DashMap;
// use glam::{Mat4, Quat, Vec3};
// use stereokit::color_named::WHITE;
// use stereokit::lifecycle::StereoKitContext;
// use stereokit::material::Material;
// use stereokit::mesh::Mesh;
// use stereokit::model::{Model, NodeId};
// use stereokit::physics::{Solid, SolidType};
// use stereokit::pose::Pose;
// use stereokit::render::RenderLayer;
// use stereokit::shader::Shader;
// use stereokit::texture::Texture;
// use uuid::Uuid;
// use cloudcafe_common::packet::{Client, ServerStatus};
// use cloudcafe_common::ports::ServerInfo;
// use stereokit_locomotion::LocomotionTracker;
// use stereokit_vrm::VrmAvatar;
// use crate::connection_manager::ConnectionManager;
// use crate::main_menu::{MainMenu, MainMenuMsg};
// use crate::resources::Resources;
// use crate::settings_menu::SettingsMenu;

mod settings_menu;
mod resources;
mod main_menu;
mod avatar_networking;
mod connection_manager;
mod other;

pub fn main() {
    other::main().unwrap();
}

//
// pub type PlayerList = Arc<Mutex<HashMap<SocketAddr, PlayerData>>>;
//
// pub type Clients = Arc<DashMap<Uuid, ClientData>>;
//
// pub struct PlayerData {
//     pub avatar: VrmAvatar,
// }
//
// pub struct ClientData {
//     pub client: Client,
//     pub player: PlayerData
// }
// impl ClientData {
//     pub fn from_client(client: Client, sk: &impl StereoKitContext) -> ClientData {
//         ClientData {
//             client,
//             player: PlayerData {
//                 avatar: VrmAvatar::load_from_file(sk, "Malek.vrm", &Shader::default(&sk)).unwrap(),
//             },
//         }
//     }
// }
//
// fn main() {
//     let sk = stereokit::Settings::default().init().unwrap();
//     let mut resources = Resources::init().unwrap();
//     let mut settings_menu = SettingsMenu::from(&resources);
//     let mut floor_solid = stereokit::physics::Solid::new(&sk, [0.0, -0.25, 0.0].into(), Quat::IDENTITY.into(), SolidType::Unaffected).unwrap();
//     floor_solid.add_box(&sk, [40.0, 1.0, 40.0].into(), 1.0, Vec3::default().into());
//     let mut floor_material = Material::create(&sk, &Shader::default(&sk)).unwrap();
//     let floor_texture = Texture::from_file(&sk, "grass.jpg", false, 0).unwrap();
//     floor_material.set_texture(&sk, "diffuse", &floor_texture).unwrap();
//     let mut floor_mesh = Mesh::gen_cube(&sk, [40.0, 1.0, 40.0], 1).unwrap();
//     let mut floor_model = Model::from_mesh(&sk, &floor_mesh, &floor_material).unwrap();
//
//     let mut main_menu = MainMenu::new(&sk);
//
//
//     let mut locomotion = LocomotionTracker::new(0.1, 1.0, 1.0);
//     locomotion.stage_pose = Pose::new([-1.0, 5.5, -1.0], Quat::IDENTITY);
//
//     let mut player_list: PlayerList = Arc::new(Mutex::new(HashMap::new()));
//     let mut connection_manager = ConnectionManager::new(String::from("Malek"), &player_list);
//     let mut connected_server_info = Arc::new(Mutex::new(None));
//
//     let mut player_avatar = VrmAvatar::load_from_file(&sk, "Malek.vrm", &Shader::default(&sk)).unwrap();
//
//
//     sk.run(|sk| {
//         locomotion.analogue_controls(sk);
//         settings_menu.exist(sk);
//         floor_model.draw(sk, floor_solid.get_pose(sk).as_matrix(), stereokit::color_named::LAWN_GREEN, RenderLayer::Layer2);
//         for (addr, player_data) in player_list.lock().unwrap().iter() {
//             player_data.avatar.draw(sk, &Pose::IDENTITY);
//         }
//         player_avatar.draw(sk, &Pose::IDENTITY);
//         player_avatar.update_ik(sk);
//         if let Some(msg) = main_menu.exist(sk) {
//             match msg {
//                 MainMenuMsg::ConnectToServer(server_info) => {
//                     println!("{:?}", server_info);
//                     connection_manager.connect(server_info.ip);
//                 }
//             }
//         }
//         if let Some(active_connection) = &mut connection_manager.connection {
//             active_connection.avatar_networking.send_avatar_packet(&player_avatar, sk);
//             active_connection.avatar_networking.update_avatars(&player_list, sk);
//
//             let mut x = active_connection.shared_server_info.lock().unwrap();
//             if let Some(server_status) = x.take() {
//                 let server_status = server_status.clone();
//                 match server_status {
//                     ServerStatus::ServerInfo(server_info) => {
//                         println!("recieved server info packet: {:?}", server_info);
//                         let _ = connected_server_info.lock().unwrap().insert(server_info.clone());
//                         for server_player_info in server_info.players {
//                             player_list.lock().unwrap().insert(server_player_info.addr,
//                             PlayerData {
//                                 avatar: VrmAvatar::load_from_file(sk, "Malek.vrm", &Shader::default(sk)).unwrap(),
//                                 username: server_player_info.username,
//                             });
//                         }
//                     }
//                     ServerStatus::Kick => {
//                         drop(x);
//                         connection_manager.disconnect();
//                     }
//                     ServerStatus::Heartbeat => {
//                         //Todo something to track if the server has crashed and stops sending heartbeats
//                     }
//                     ServerStatus::ClientDisconnected(client_addr) => {
//                         if let Some(player_data) =  player_list.lock().unwrap().remove(&client_addr) {
//                             println!("player: {}, addr: {} disconnected", player_data.username, client_addr);
//                         }
//                     }
//                     ServerStatus::ClientConnected((client_addr, client_info)) => {
//                         let player_data = PlayerData {
//                             avatar: VrmAvatar::load_from_file(sk, "Malek.vrm", &Shader::default(sk)).unwrap(),
//                             username: client_info.name.clone(),
//                         };
//                         println!("player: {}, addr: {} connected", player_data.username, client_addr);
//                         player_list.lock().unwrap().insert(client_addr, player_data);
//                     }
//                 }
//             }
//         }
//     }, |_| {});
// }
