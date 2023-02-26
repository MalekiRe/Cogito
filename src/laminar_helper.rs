use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use color_eyre::Result;

#[derive(Clone)]
pub struct ConnectionTrigger(Arc<Mutex<bool>>);
impl ConnectionTrigger {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(false)))
    }
    pub fn disconnect(&self) {
        *self.0.lock().unwrap() = true;
    }
    pub fn disconnected(&self) -> bool {
        *self.0.lock().unwrap()
    }
}

pub struct ConnectionHandler {
    thread: JoinHandle<Result<()>>,
    address: SocketAddr,
    connection_trigger: ConnectionTrigger,
}

impl ConnectionHandler {
    pub fn spawn_simple(mut callback: impl FnOnce(ConnectionTrigger, Option<Duration>) -> Result<()> + Send + 'static, address: SocketAddr, sleep_duration: Option<Duration>) -> Self {
        let connection_trigger = ConnectionTrigger::new();
        let conn_trigg = connection_trigger.clone();
        Self {
            thread: thread::spawn(move || callback(conn_trigg, sleep_duration)),
            address,
            connection_trigger,
        }
    }
    pub fn spawn<'a, T: Send + 'static>(looping_callback: impl Fn(&mut T) -> Result<()> + Send + 'static, address: SocketAddr, sleep_duration: Option<Duration>, args: T) -> Self {
        let connection_trigger = ConnectionTrigger::new();
        let conn_trigg = connection_trigger.clone();
        Self {
            thread: thread::spawn(move || {
                let mut args = args;
                loop {
                    if conn_trigg.disconnected() {
                        return Ok(());
                    }
                    if let Some(duration) = sleep_duration {
                        thread::sleep(duration);
                    }
                    match looping_callback(&mut args) {
                        Ok(_) => {}
                        Err(err) => { return Err(err); }
                    }
                }
            }),
            address,
            connection_trigger,
        }
    }
    pub fn disconnect(self) -> Result<()> {
        self.connection_trigger.disconnect();
        self.thread.join().expect("joined thread panicked, shouldn't happpen, return Result<()> instead")
    }
    pub fn address(&self) -> SocketAddr {
        self.address
    }
}
/*
loop {
                if conn_trigg.disconnected() {
                    return Ok(());
                }
                if let Some(duration) = sleep_duration {
                    thread::sleep(duration)
                }
                match looping_callback() {
                    Ok(_) => {}
                    Err(err) => { return Err(err); }
                }
            }
 */