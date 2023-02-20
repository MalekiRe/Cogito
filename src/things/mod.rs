mod server;
mod client;

use std::thread;
use color_eyre::Result;

pub fn run() -> Result<()> {
    let t1 = thread::spawn(|| server::server("74.207.246.102:8888"));
    //std::thread::sleep(std::time::Duration::from_millis(30));
    //let t2 = thread::spawn(|| client::client("74.207.246.102:8888"));
    //let t3 = thread::spawn(|| client::client("74.207.246.102:25565"));
    t1.join().unwrap().unwrap();
    //t2.join().unwrap().unwrap();
    //t3.join().unwrap().unwrap();
    Ok(())
}