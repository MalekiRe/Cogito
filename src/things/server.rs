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
    loop {
        for event in server.step() {
            match event {
                Event::Connect(_) => {}
                Event::Disconnect(_) => {}
                Event::Receive(original_address, packet) => {
                    let addresses = server.client_addresses();
                    for address in &addresses {
                        // if &original_address == address {
                        //     continue;
                        // }
                        server.client(address).unwrap().borrow_mut()
                            .send(packet.clone(), 0, SendMode::TimeSensitive)
                    }
                }
                Event::Error(_, _) => {}
            }
            server.flush();
        }
    }
    Ok(())
}