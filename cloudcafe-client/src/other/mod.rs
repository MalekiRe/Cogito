mod connection_manager;
mod info_connection;
mod avatar_connection;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use color_eyre::Result;
use dashmap::DashMap;
use log::info;
use stereokit::lifecycle::StereoKitContext;
use stereokit::pose::Pose;
use stereokit::shader::Shader;
use uuid::Uuid;
use cloudcafe_common::packet::{Client, ClientData, ServerStatus};
use stereokit_vrm::VrmAvatar;
use crate::main_menu::{MainMenu, MainMenuMsg};
use crate::other::avatar_connection::AvatarPos;
use crate::other::connection_manager::ConnectionManager;
use crate::resources::Resources;

pub type Clients = Arc<DashMap<Uuid, Client>>;
pub type Players = HashMap<Uuid, Player>;
/// this is our singleton instance of our player in this game.

pub type SinglePlayer = Rc<RefCell<Player>>;

pub fn main() -> Result<()> {
    simple_logger::init().unwrap();
    let sk = stereokit::Settings::default().disable_unfocused_sleep(true).init()?;
    let connected_server_info = Arc::new(Mutex::new(None));
    let mut main_menu = MainMenu::new(&sk, connected_server_info.clone());
    let client_data = ClientData {
        name: "malek".to_string(),
        uuid: Uuid::new_v4(),
    };

    let clients = Arc::new(DashMap::new());
    let mut players: HashMap<Uuid, Player> = HashMap::new();

    let mut player = Player {
        uuid: client_data.uuid.clone(),
        avatar: VrmAvatar::load_from_file(&sk, "Malek.vrm", &Shader::default(&sk))?,
    };

    let mut connection_manager = ConnectionManager::new(client_data.clone(), &clients);

    let mut timer = 0;
    sk.run(|sk| {



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
    players.insert(client.data.uuid, Player {
        uuid: client.data.uuid,
        avatar: VrmAvatar::load_from_file(sk, "Malek.vrm", &Shader::default(sk)).unwrap(),
    });
    clients.insert(client.data.uuid, client);
}

// client is stuff for over the network direct communication and information, player is stuff that is local to each instance and bits and pieces may or may not be synced over the network
pub struct Player {
    uuid: Uuid,
    avatar: VrmAvatar,
}