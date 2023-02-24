mod server;
mod client;
pub mod ports;
pub mod packet;

use std::io::{stdin};
use std::thread;
use std::time::Instant;
use color_eyre::Result;
const SERVER: &str = "127.0.0.1:12351";

fn main() -> Result<()> {
    let stdin = stdin();

    println!("Please type in `server` or `client`.");

    let mut s = String::new();
    stdin.read_line(&mut s)?;

    if s.starts_with('s') {
        println!("Starting server..");
        server::server()
    } else {
        println!("Starting client..");
        client::client()
    }
}