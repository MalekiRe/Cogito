// use std::collections::HashMap;
// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::thread::JoinHandle;
// use std::time::{Duration, Instant};
// use crossbeam::channel::{Receiver, Sender};
// use laminar::{Packet, Socket, SocketEvent};
// use stereokit::lifecycle::{StereoKitContext, StereoKitDraw};
// use stereokit::model::NodeId;
// use stereokit::pose::Pose;
// use cloudcafe_common::ports::AVATAR_PACKET_PORT;
// use stereokit_vrm::VrmAvatar;
// use crate::{PlayerData, PlayerList};
//
// pub struct AvatarNetworkingHandler {
//     tx: Sender<Packet>,
//     server_sock_addr: SocketAddr,
//     rx_manager: Arc<Mutex<bool>>,
//     rx_thread: JoinHandle<()>,
//     socket_thread: JoinHandle<()>,
//     player_model_positions: Arc<Mutex<HashMap<SocketAddr, Vec<(NodeId, Pose)>>>>
// }
//
// impl AvatarNetworkingHandler {
//     const DURATION: Duration = Duration::from_millis(1);
//     pub fn new(player_list: &PlayerList, server_address: Ipv4Addr) -> Self {
//         let player_list = player_list.clone();
//         let player_model_positions = Arc::new(Mutex::new(HashMap::new()));
//         for item in player_list.lock().unwrap().keys() {
//             player_model_positions.lock().unwrap().insert(*item, vec![]);
//         }
//         let p_m_p = player_model_positions.clone();
//         let socket = Socket::bind(format!("0.0.0.0:{}", portpicker::pick_unused_port().expect("No unused ports avaliable for avatar networking"))).expect("Unable to init avatar networking port");
//         let rx_manager = Arc::new(Mutex::new(false));
//         let r_m = rx_manager.clone();
//         let (rx, tx) = (socket.get_event_receiver(), socket.get_packet_sender());
//         let rx_thread = thread::spawn(move || rx_networking(p_m_p, rx, r_m));
//         let r_m = rx_manager.clone();
//         let socket_thread = thread::spawn(move || socket_thread(socket, Self::DURATION, r_m));
//         let server_sock_addr = SocketAddr::new(IpAddr::V4(server_address), AVATAR_PACKET_PORT);
//         Self {
//             tx,
//             server_sock_addr,
//             rx_manager,
//             rx_thread,
//             socket_thread,
//             player_model_positions,
//         }
//     }
//     pub fn send_avatar_packet(&mut self, avatar: &VrmAvatar, sk: &StereoKitDraw) {
//         let bytes = bincode::serialize(&avatar.get_nodes_and_poses(sk)).unwrap();
//         let packet = Packet::unreliable(self.server_sock_addr, bytes);
//         self.tx.send(packet).unwrap();
//     }
//     pub fn update_avatars(&self, player_list: &PlayerList, sk: &impl StereoKitContext) {
//         for (addr, data) in self.player_model_positions.lock().unwrap().iter() {
//             match player_list.lock().unwrap().get_mut(addr) {
//                 None => {
//                     //todo remove the player addr somehow idk
//                 }
//                 Some(player_data) => {
//                     player_data.avatar.set_nodes_and_poses(data.clone());
//                 }
//             }
//         }
//     }
//     pub fn disconnect(&mut self) {
//         *self.rx_manager.lock().unwrap() = true;
//     }
//     pub fn resolve_connections(self) {
//         todo!()
//     }
//     pub fn block_on_disconnect(self) {
//         todo!()
//     }
// }
//
// fn socket_thread(mut socket: Socket, duration: Duration, rx_manager: Arc<Mutex<bool>>) {
//     loop {
//         thread::sleep(duration);
//         socket.manual_poll(Instant::now());
//         if *rx_manager.lock().unwrap() {
//             return;
//         }
//     }
// }
// fn rx_networking(player_model_positions: Arc<Mutex<HashMap<SocketAddr, Vec<(NodeId, Pose)>>>>, rx: Receiver<SocketEvent>, rx_manager: Arc<Mutex<bool>>) {
//     loop {
//         if *rx_manager.lock().unwrap() {
//             return;
//         }
//         for msg in rx.try_iter() {
//             match msg {
//                 SocketEvent::Packet(packet) => {
//                     todo!()
//                 }
//                 _ => {}
//             }
//         }
//     }
// }