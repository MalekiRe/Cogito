// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::thread::JoinHandle;
// use std::time::{Duration, Instant};
// use laminar::{Packet, Socket, SocketEvent};
// use cloudcafe_common::laminar_helper::{ConnectionHandler, ConnectionTrigger};
// use cloudcafe_common::packet::{ClientStatus, ServerStatus};
// use cloudcafe_common::ports::{CLIENT_INFO_PORT, SERVER_STATUS_PORT, ServerInfo};
// use crate::avatar_networking::AvatarNetworkingHandler;
// use crate::PlayerList;
//
// pub struct ConnectionManager {
//     pub connection: Option<ActiveConnection>,
// }
//
// pub struct ActiveConnection {
//     info_connection: ConnectionHandler,
//     // avatar_connection: ConnectionHandler,
//
// }
//
// impl ActiveConnection {
//
// }
//
// impl ConnectionManager {
//     pub fn new(username: String, player_list: &PlayerList) -> Self {
//         Self { username, player_list: player_list.clone(), connection: None }
//     }
//     pub fn connect(&mut self, server_addr: Ipv4Addr) {
//         match self.connection.take() {
//             Some(active_connection) => {
//                 active_connection.disconnect();
//             }
//             _ => {},
//         }
//         println!("connecting to: {}", server_addr);
//         self.connection = Some(ActiveConnection::connect(self.username.clone(), server_addr, &self.player_list));
//     }
//     pub fn disconnect(&mut self) {
//         match self.connection.take() {
//             None => {}
//             Some(connection) => {
//                 connection.disconnect();
//             }
//         }
//     }
// }
// fn server_info(username: String, shared_server_info: Arc<Mutex<Option<ServerStatus>>>, server_info_manager: Arc<Mutex<bool>>, heartbeat_manager: Arc<Mutex<bool>>, server_addr: Ipv4Addr) {
//     const SLEEP_DURATION: Duration = Duration::from_millis(4);
//     const HEARTBEAT_FREQUENCY: Duration = Duration::from_millis(10);
//     let mut socket = Socket::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), portpicker::pick_unused_port().expect("No ports free"))).expect("couldn't open server info socket");
//     let connection_packet = Packet::reliable_unordered(SocketAddr::V4(SocketAddrV4::new(server_addr, CLIENT_INFO_PORT)), bincode::serialize(&ClientStatus::Connect(ClientInfo { name: username })).unwrap());
//     println!("sending connection packet");
//     socket.send(connection_packet).unwrap();
//     let heartbeat_packet = Packet::reliable_unordered(
//         SocketAddrV4::new(server_addr, CLIENT_INFO_PORT).into(),
//         bincode::serialize(&ClientStatus::Heartbeat).expect("couldn't serialize heartbeat packet???? something is *very* wrong")
//     );
//     let packet_sender = socket.get_packet_sender();
//     let _t = thread::spawn(move || {
//        loop {
//            thread::sleep(HEARTBEAT_FREQUENCY);
//            match packet_sender.send(heartbeat_packet.clone()) {
//                Ok(_) => (),
//                Err(_) =>
//                    println!("couldn't send heartbeat packet to server"),
//            }
//            if *heartbeat_manager.lock().expect("couldn't lock on heartbeat_manager bool") {
//                return;
//            }
//        }
//     });
//     let rx = socket.get_event_receiver();
//     loop {
//         socket.manual_poll(Instant::now());
//         thread::sleep(SLEEP_DURATION);
//         match rx.try_recv() {
//             Ok(SocketEvent::Packet(packet)) => {
//                 let server_status = bincode::deserialize(packet.payload()).unwrap();
//                 let _ = shared_server_info.lock().unwrap().insert(server_status);
//             }
//             _ => {}
//         }
//         if *server_info_manager.lock().unwrap() {
//             socket.get_packet_sender().send(Packet::reliable_unordered(SocketAddrV4::new(server_addr, CLIENT_INFO_PORT).into(), bincode::serialize(&ClientStatus::Disconnect).unwrap())).unwrap();
//             for _ in 0..5 {
//                 thread::sleep(SLEEP_DURATION);
//                 socket.manual_poll(Instant::now());
//             }
//             _t.join().unwrap();
//             return;
//         }
//     }
// }