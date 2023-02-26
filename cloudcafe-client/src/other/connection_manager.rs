use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use cloudcafe_common::laminar_helper::ConnectionHandler;
use cloudcafe_common::packet::{Client, ClientAddresses, ClientData, ServerStatus};
use crate::other::{avatar_connection, Clients, Player, Players, SinglePlayer};
use color_eyre::Result;
use crossbeam::channel::{Receiver, Sender};
use laminar::Socket;
use cloudcafe_common::ports::{AVATAR_PACKET_PORT, CLIENT_INFO_PORT};
use crate::other::info_connection::info_connection;
use log::{warn, info};
use stereokit::model::NodeId;
use stereokit::pose::Pose;
use uuid::Uuid;
use crate::other::avatar_connection::{avatar_connection, AvatarPos};

pub struct ActiveConnection {
    info_connection: ConnectionHandler,
    avatar_connection: ConnectionHandler,
    single_client: Client,
    pub(crate) server_status_rx: Receiver<ServerStatus>,
    pub(crate) other_avatar_rx: Receiver<AvatarPos>,
    pub(crate) this_avatar_tx: Sender<AvatarPos>,
}
impl ActiveConnection {
    pub fn connect(client_data: ClientData, clients: &Clients, server_address: Ipv4Addr) -> Self {
        // TODO, we need to use the actual real ip address locally i think, instead of 0.0.0.0 or 127.0.0.1, like, whatever the address is.
        // TODO It is fine for now cause we are just running locally, but change this later don't forget!

        let server_info_address = SocketAddr::V4(SocketAddrV4::new(server_address, CLIENT_INFO_PORT));
        let server_avatar_address = SocketAddr::V4(SocketAddrV4::new(server_address, AVATAR_PACKET_PORT));
        let client_info_port = portpicker::pick_unused_port().expect("unbound port not found");
        let client_avatar_net_port = portpicker::pick_unused_port().expect("unbound port not found");
        let client_info_address = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), client_info_port);
        let client_avatar_networking_address = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), client_avatar_net_port);

        let (server_status_tx, server_status_rx) = crossbeam::channel::unbounded();
        let (other_avatar_tx, other_avatar_rx) = crossbeam::channel::unbounded();
        let (this_avatar_tx, this_avatar_rx) = crossbeam::channel::unbounded();

        let avatar_networking_socket = Socket::bind(client_avatar_networking_address.clone()).unwrap();
        let (avatar_net_tx, avatar_net_rx) = (avatar_networking_socket.get_packet_sender(), avatar_networking_socket.get_event_receiver());

        let client = Client {
            data: client_data.clone(),
            addrs: ClientAddresses { info_addr: SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), client_info_port), avatar_networking_addr: SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), client_avatar_net_port) },
        };
        let c = client.clone();
        info!("client initialized: {:?}", client);
        Self {
            info_connection: ConnectionHandler::spawn_simple(move |connection_trigger, sleep_duration| {
                info_connection(connection_trigger, sleep_duration, c.clone(), Socket::bind(client_info_address).unwrap(), server_info_address.clone(), server_status_tx.clone())
            }, client_info_address, Some(Duration::from_millis(10))),
            avatar_connection: ConnectionHandler::spawn(avatar_connection::avatar_connection, client_avatar_networking_address, None,
                                                        (avatar_networking_socket, other_avatar_tx.clone(), avatar_net_rx, avatar_net_tx, this_avatar_rx.clone(), server_avatar_address.clone())
            ),
            single_client: client,
            server_status_rx,
            other_avatar_rx,
            this_avatar_tx,
        }
    }
    pub fn disconnect(self) -> Result<()> {
        self.info_connection.disconnect()?;
        Ok(())
    }
}

pub struct ConnectionManager {
    pub(crate) connection: Option<ActiveConnection>,
    clients: Clients,
    client_data: ClientData,
}

impl ConnectionManager {
    pub fn new(client_data: ClientData, clients: &Clients) -> Self {
        Self {
            connection: None,
            clients: clients.clone(),
            client_data,
        }
    }
    pub fn connect(&mut self, server_address: Ipv4Addr) {
        if let Some(connection) = self.connection.take() {
            connection.disconnect().unwrap();
        }
        self.connection = Some(ActiveConnection::connect(self.client_data.clone(), &self.clients, server_address));
    }
    pub fn disconnect(&mut self) {
        if let Some(connection) = self.connection.take() {
            connection.disconnect().unwrap();
        } else {
            warn!("tried to disconnect but no active connection")
        }
    }
}