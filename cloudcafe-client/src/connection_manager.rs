use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use laminar::{Packet, Socket, SocketEvent};
use cloudcafe_common::packet::{ClientInfo, ClientStatus, ServerStatus};
use cloudcafe_common::ports::{CLIENT_INFO_PORT, SERVER_STATUS_PORT, ServerInfo};

pub struct ConnectionManager {
    username: String,
    pub connection: Option<ActiveConnection>,
}

pub struct ActiveConnection {
    heartbeat_manager: Arc<Mutex<bool>>,
    server_info_thread: JoinHandle<()>,
    server_info_manager: Arc<Mutex<bool>>,
    pub shared_server_info: Arc<Mutex<Option<ServerStatus>>>,
}

impl ActiveConnection {
    pub fn connect(username: String, server_addr: Ipv4Addr) -> Self {
        let heartbeat_manager = Arc::new(Mutex::new(false));
        let h_m = heartbeat_manager.clone();
        let server_info_manager = Arc::new(Mutex::new(false));
        let shared_server_info = Arc::new(Mutex::new(None));
        let s_i_m = server_info_manager.clone();
        let s_s_i = shared_server_info.clone();
        let server_info_thread = thread::spawn(move || { server_info(username, s_s_i, s_i_m, h_m, server_addr.clone()) });
        Self {
            heartbeat_manager,
            server_info_thread,
            server_info_manager,
            shared_server_info,
        }
    }
    pub fn disconnect(self) {
        *self.heartbeat_manager.lock().unwrap() = true;
        *self.server_info_manager.lock().unwrap() = true;

        // Because of the delay we aren't gonna join the threads
        self.server_info_thread.join().unwrap();
    }
}

impl ConnectionManager {
    pub fn new(username: String) -> Self {
        Self { username, connection: None }
    }
    pub fn connect(&mut self, server_addr: Ipv4Addr) {
        match self.connection.take() {
            Some(active_connection) => {
                active_connection.disconnect();
            }
            _ => {},
        }
        println!("connecting to: {}", server_addr);
        self.connection = Some(ActiveConnection::connect(self.username.clone(), server_addr));
    }
    pub fn disconnect(&mut self) {
        match self.connection.take() {
            None => {}
            Some(connection) => {
                connection.disconnect();
            }
        }
    }
}
fn server_info(username: String, shared_server_info: Arc<Mutex<Option<ServerStatus>>>, server_info_manager: Arc<Mutex<bool>>, heartbeat_manager: Arc<Mutex<bool>>, server_addr: Ipv4Addr) {
    const SLEEP_DURATION: Duration = Duration::from_millis(4);
    const HEARTBEAT_FREQUENCY: Duration = Duration::from_millis(10);
    let mut socket = Socket::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), portpicker::pick_unused_port().expect("No ports free"))).expect("couldn't open server info socket");
    let connection_packet = Packet::reliable_unordered(SocketAddr::V4(SocketAddrV4::new(server_addr, CLIENT_INFO_PORT)), bincode::serialize(&ClientStatus::Connect(ClientInfo { name: username })).unwrap());
    println!("sending connection packet");
    socket.send(connection_packet).unwrap();
    let heartbeat_packet = Packet::reliable_unordered(
        SocketAddrV4::new(server_addr, CLIENT_INFO_PORT).into(),
        bincode::serialize(&ClientStatus::Heartbeat).expect("couldn't serialize heartbeat packet???? something is *very* wrong")
    );
    let packet_sender = socket.get_packet_sender();
    let _t = thread::spawn(move || {
       loop {
           thread::sleep(HEARTBEAT_FREQUENCY);
           match packet_sender.send(heartbeat_packet.clone()) {
               Ok(_) => (),
               Err(_) =>
                   println!("couldn't send heartbeat packet to server"),
           }
           if *heartbeat_manager.lock().expect("couldn't lock on heartbeat_manager bool") {
               return;
           }
       }
    });
    let rx = socket.get_event_receiver();
    loop {
        socket.manual_poll(Instant::now());
        thread::sleep(SLEEP_DURATION);
        match rx.try_recv() {
            Ok(SocketEvent::Packet(packet)) => {
                let server_status = bincode::deserialize(packet.payload()).unwrap();
                let _ = shared_server_info.lock().unwrap().insert(server_status);
            }
            _ => {}
        }
        if *server_info_manager.lock().unwrap() {
            socket.get_packet_sender().send(Packet::reliable_unordered(SocketAddrV4::new(server_addr, CLIENT_INFO_PORT).into(), bincode::serialize(&ClientStatus::Disconnect).unwrap())).unwrap();
            for _ in 0..5 {
                thread::sleep(SLEEP_DURATION);
                socket.manual_poll(Instant::now());
            }
            _t.join().unwrap();
            return;
        }
    }
}