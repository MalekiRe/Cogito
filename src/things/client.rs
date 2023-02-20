use std::convert::Into;
use std::io::Read;
use std::{net, thread};
use std::net::{SocketAddr, SocketAddrV4};
use std::time::Instant;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use glam::Vec3;
use laminar::{Packet, SocketEvent};
use serde::{Deserialize, Serialize};
use stereokit::input::StereoKitInput;
use stereokit::lifecycle::DisplayMode;
use stereokit::microphone::Microphone;
use stereokit::sound::{SoundStream, SoundT};
use stereokit_locomotion::LocomotionTracker;

const CLIENT_ADDRESS: &'static str = "127.0.0.1:0245";

pub fn client(server_address: SocketAddrV4, client_address: SocketAddrV4) -> Result<()> {
    let mut socket = laminar::Socket::bind(client_address)?;
    let (mut sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let sk = stereokit::Settings::default().init()?;
    let devices = stereokit::microphone::Microphone::device_count();
    for i in 0..devices {
        println!("{}, {}", Microphone::device_name(i), i);
    }
    let mic = Microphone::new(1);
    mic.start();
    //let sound = mic.get_stream()?;
    //sound.play_sound(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let mut locomotion_tracker = LocomotionTracker::new(1.0, 1.0, 1.0);
    let sound = mic.get_stream().unwrap();
    //sound.play_sound(Vec3::new(0.0, 0.0, 0.0), 10.0);
    let mut num = 0;
    //let _t = thread::spawn(move || socket.start_polling());
    sk.run(|sk| {
        socket.manual_poll(Instant::now());
        match receiver.try_recv() {
            Ok(SocketEvent::Packet(mut pack)) => {
                let samples_with_pos: SamplesWithPos = bincode::deserialize(pack.payload()).unwrap();
                let sound_stream = SoundStream::create(samples_with_pos.samples.len() as f32);
                sound_stream.write_samples(samples_with_pos.samples.as_slice());
                sound_stream.play_sound(samples_with_pos.pos, 1.0);
            }
            _ => {}
        }
        num += 1;
        if num == 50 {
            println!("sending sounds?");
            let len = sound.unread_samples();
            let mut samples_with_pos = SamplesWithPos::new(sk.input_head().position.into(), len);
            sound.read_samples(samples_with_pos.samples.as_mut_slice());
            locomotion_tracker.locomotion_update(sk);
            let bytes = bincode::serialize(&samples_with_pos).unwrap();
            sender.send(Packet::reliable_unordered(SocketAddr::from(server_address), bytes)).unwrap();
            //client.send(bytes.into_boxed_slice(), 0, SendMode::Persistent);
            num = 0;
        }
    }, |_|{});
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplesWithPos {
    pos: Vec3,
    pub samples: Vec<f32>,
}

impl SamplesWithPos {
    pub fn new(pos: Vec3, size: u64) -> Self {
        let mut samples = Vec::new();
        samples.resize(size as usize, 0.0);
        Self {
            pos,
            samples,
        }
    }
}