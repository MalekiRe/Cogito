mod server;
mod client;

use std::net::SocketAddrV4;
use std::str::FromStr;
use std::thread;
use color_eyre::Result;

pub fn run() -> Result<()> {
    let t1 = thread::spawn(|| server::server(SocketAddrV4::from_str("127.0.0.1:8808").unwrap()));
    std::thread::sleep(std::time::Duration::from_millis(30));
    //let t2 = thread::spawn(|| client::client(SocketAddrV4::from_str("127.0.0.1:8808").unwrap(), SocketAddrV4::from_str("127.0.0.1:8808").unwrap()));
    //let t3 = thread::spawn(|| client::client("74.207.246.102:25565"));
    t1.join().unwrap().unwrap();
    //t2.join().unwrap().unwrap();
    //t3.join().unwrap().unwrap();
    Ok(())
}