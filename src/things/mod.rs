mod server;
mod client;

use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::thread;
use color_eyre::Result;

pub fn run() -> Result<()> {
    let stdin = std::io::stdin();

    println!("Please type in `server` or `client`.");

    let mut s = String::new();
    stdin.read_line(&mut s)?;


    if s.starts_with('s') {
        server::server(SocketAddrV4::from_str("127.0.0.1:1248").unwrap().into()).unwrap();
    }
    else {
        let s_addr = SocketAddrV4::from_str("127.0.0.1:12351").unwrap();
        let c_addr = SocketAddrV4::from_str("127.0.0.1:1249").unwrap();
        client::client(s_addr.into(), c_addr.into()).unwrap();
    }
    //
    //std::thread::sleep(std::time::Duration::from_millis(30));
    //let t2 = thread::spawn(|| client::client(SocketAddrV4::from_str("74.207.246.102:8808").unwrap(), SocketAddrV4::from_str("127.0.0.1:8808").unwrap()));
    //let t3 = thread::spawn(|| client::client("74.207.246.102:25565"));
    //t1.join().unwrap().unwrap();
    //t2.join().unwrap().unwrap();
    //t3.join().unwrap().unwrap();
    Ok(())
}