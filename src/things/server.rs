use std::net;
use std::net::SocketAddr;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use uflow::SendMode;
use uflow::server::Event;

pub fn server<Address: net::ToSocketAddrs>(server_address: Address) -> Result<()> {
    let mut server = uflow::server::Server::bind(server_address, uflow::server::Config{
        max_total_connections: 5,
        max_active_connections: 5,
        enable_handshake_errors: false,
        endpoint_config: Default::default(),
    })?;
    let mut client_addresses = vec![];
    loop {
        for event in server.step() {
            match event {
                Event::Connect(connection) => {
                    println!("connected: {}", connection);
                    client_addresses.push(connection);
                }
                Event::Disconnect(dissconnect) => {
                    println!("disconnected: {}", dissconnect);
                }
                Event::Receive(original_address, packet) => {
                    for address in &client_addresses {
                        server.client(address).unwrap().borrow_mut()
                            .send(packet.clone(), 0, SendMode::Persistent)
                    }
                }
                Event::Error(_, _) => {}
            }
            server.flush();
        }
    }
    Ok(())
}