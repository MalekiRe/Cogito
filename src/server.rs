use std::thread;
use std::time::Duration;
use crate::SERVER;
use color_eyre::Result;

pub fn server() -> Result<()> {
    let server_address = "74.207.246.102:8888";
    let config = Default::default();

    // Create a server object
    let mut server = uflow::server::Server::bind(server_address, config).unwrap();

    let mut client_addresses = Vec::new();
    loop {
        // Process inbound UDP frames and handle events
        for event in server.step() {
            //server.flush();
            match event {
                uflow::server::Event::Connect(client_address) => {
                    println!("[{:?}] connected", client_address);
                    client_addresses.push(client_address);
                }
                uflow::server::Event::Disconnect(client_address) => {
                    println!("[{:?}] disconnected", client_address);
                }
                uflow::server::Event::Error(client_address, err) => {
                    println!("[{:?}] error: {:?}", client_address, err);
                }
                uflow::server::Event::Receive(client_address, packet_data) => {
                    //let packet_data_utf8 = std::str::from_utf8(&packet_data).unwrap();
                    //let mut client = server.client(&client_address).unwrap().borrow_mut();
                    for (num, address) in client_addresses.iter().enumerate() {
                         //if address != &client_address {
                            server.client(address).unwrap().borrow_mut().send(
                                packet_data.clone(), num, uflow::SendMode::Unreliable
                            );
                            //server.flush();
                        //}
                    }
                    // Echo the packet reliably on channel 0
                    //client.send(packet_data, 0, uflow::SendMode::Reliable);

                    // Echo the reverse of the packet unreliably on channel 1

                    //client.send(reversed_string.as_bytes().into(), 1, uflow::SendMode::Unreliable);
                }
            }
        }

        // Flush outbound UDP frames
        //server.flush();
        //std::thread::sleep(Duration::from_nanos(2));
        server.flush();
        std::thread::sleep(std::time::Duration::from_micros(30));
        //std::thread::sleep(std::time::Duration::from_millis(30));
    }
    Ok(())
}