use std::collections::HashMap;
use std::io::stdin;
use std::net::SocketAddr;
use std::time::Instant;
use glam::Vec3;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use stereokit::input::StereoKitInput;
use stereokit::lifecycle::{DisplayMode, StereoKitContext};
use stereokit::microphone::Microphone;
use stereokit_locomotion::LocomotionTracker;
use serde::{Serialize, Deserialize};
use uflow::SendMode;
use crate::SERVER;
use color_eyre::Result;
use stereokit::sound::{SoundInstance, SoundStream, SoundT};

pub fn client() -> Result<()> {
    let server_address = "74.207.246.102:8888";
    let config = Default::default();

    // Create a client object
    let mut client = uflow::client::Client::connect(server_address, config).unwrap();

    println!("Type a message and press Enter to send. Send `Bye!` to quit.");

    let stdin = stdin();
    let mut s_buffer = String::new();

    let sk = stereokit::Settings::default().init().unwrap();
    let devices = Microphone::device_count();
    for i in 0..devices {
        println!("{}, {}", Microphone::device_name(i), i);
    }
    let mic = Microphone::new(1);
    mic.start();
    let mut locomotion_tracker = LocomotionTracker::new(1.0, 1.0, 1.0);
    let sound = mic.get_stream().unwrap();
    let mut sample_counter = 0;

    let mut sounds: HashMap<SocketAddr, (SoundStream, SoundInstance)> = HashMap::new();

    sk.run(|sk| {
        locomotion_tracker.locomotion_update(sk);
        sample_counter += 1;
        if sample_counter >= 10 {
            sample_counter = 0;
            let len = sound.unread_samples();
            //println!("len: {}", len);
            let mut samples_with_pos = SamplesWithPos::new(sk.input_head().position.into(), client.remote_address(), len);
            sound.read_samples(samples_with_pos.samples.as_mut_slice());
            let bytes = bincode::serialize(&samples_with_pos).unwrap();
            //println!("sending bytes: {:#?}", bytes);
            client.send(bytes.into_boxed_slice(), 0, SendMode::Reliable);
        }

        for event in client.step() {
            match event {
                uflow::client::Event::Connect => {
                    println!("connected to server");
                }
                uflow::client::Event::Disconnect => {
                    println!("disconnected from server");
                }
                uflow::client::Event::Error(err) => {
                    println!("server connection error: {:?}", err);
                }
                uflow::client::Event::Receive(packet_data) => {
                    //println!("reciving data");
                    //let packet_data_utf8 = std::str::from_utf8(&packet_data).unwrap();
                    let samples_with_pos: SamplesWithPos = bincode::deserialize(&*packet_data).unwrap();
                    if sounds.contains_key(&samples_with_pos.client) {
                        let (stream, instance) = sounds.get(&samples_with_pos.client).unwrap();
                        stream.write_samples(samples_with_pos.samples.as_slice());
                        instance.set_position(samples_with_pos.pos);
                    } else {
                        let sound_stream = SoundStream::create(3.0);
                        sound_stream.write_samples(samples_with_pos.samples.as_slice());
                        let instance = sound_stream.play_sound(samples_with_pos.pos, 1.0);
                        sounds.insert(samples_with_pos.client, (sound_stream, instance));
                    }
                }
            }
        }
        client.flush();
    }, |_| {});
    // loop {
    // }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplesWithPos {
    pos: Vec3,
    client: SocketAddr,
    pub samples: Vec<f32>,
}
impl SamplesWithPos {
    pub fn new(pos: Vec3, addr: SocketAddr, size: u64) -> Self {
        let mut samples = Vec::new();
        samples.resize(size as usize, 0.0);
        Self {
            pos,
            client: addr,
            samples,
        }
    }
}