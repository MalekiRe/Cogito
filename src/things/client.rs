use std::io::Read;
use std::net;
use std::net::SocketAddr;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use glam::Vec3;
use serde::{Deserialize, Serialize};
use stereokit::input::StereoKitInput;
use stereokit::microphone::Microphone;
use stereokit::sound::{SoundStream, SoundT};
use uflow::client::Event;
use uflow::SendMode;
use stereokit_locomotion::LocomotionTracker;

pub fn client<Address: net::ToSocketAddrs>(server_address: Address) -> Result<()> {
    let mut client = uflow::client::Client::connect(server_address, Default::default())?;
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
    sound.play_sound(Vec3::new(0.0, 0.0, 0.0), 10.0);
    sk.run(|sk| {
        for event in client.step() {
            match event {
                Event::Connect => {
                    println!("connected!");
                }
                Event::Disconnect => {}
                Event::Receive(mut sound) => {
                    let samples_with_pos: SamplesWithPos = bincode::deserialize(sound.as_mut()).unwrap();
                    let sound_stream = SoundStream::create(samples_with_pos.samples.len() as f32);
                    sound_stream.write_samples(samples_with_pos.samples.as_slice());
                    sound_stream.play_sound(samples_with_pos.pos, 1.0);
                }
                Event::Error(_) => {}
            }
        }
        
        let len = sound.unread_samples();
        let mut samples_with_pos = SamplesWithPos::new(sk.input_head().position.into(), len);
        sound.read_samples(samples_with_pos.samples.as_mut_slice());
        locomotion_tracker.locomotion_update(sk);
        let bytes = bincode::serialize(&samples_with_pos).unwrap();
        client.send(bytes.into_boxed_slice() ,0, SendMode::TimeSensitive);
        client.flush();
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