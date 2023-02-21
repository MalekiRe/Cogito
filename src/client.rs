use std::collections::HashMap;
use std::io::stdin;
use std::net::SocketAddr;
use std::thread;
use std::time::{Duration, Instant};
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
use crossbeam::channel::TryRecvError;
use stereokit::lifecycle::DisplayMode::Flatscreen;
use stereokit::model::NodeId;
use stereokit::pose::Pose;
use stereokit::shader::Shader;
use stereokit::sound::{SoundInstance, SoundStream, SoundT};
use stereokit_vrm::VrmAvatar;

pub fn laminar_version() {
    let server_addr = "74.207.246.102:8888".parse().unwrap();
    let avatar_server_addr = "74.207.246.102:8889".parse().unwrap();
    //let server_addr = "127.0.0.1:8888".parse().unwrap();
    //let mut socket = Socket::bind_any().unwrap();
    let r: u8 = rand::random();
    let mut socket = Socket::bind(format!("0.0.0.0:6{}", r).as_str()).unwrap();
    let mut avatar_socket = Socket::bind(format!("0.0.0.0:7{}", r).as_str()).unwrap();
    let client_addr = socket.local_addr().unwrap();
    let (mut avatar_rx, avatar_tx) = (avatar_socket.get_event_receiver(), avatar_socket.get_packet_sender());
    let (mut rx, tx) = (socket.get_event_receiver(), socket.get_packet_sender());
    let _t = thread::spawn(move ||  socket.start_polling_with_duration(None));
    let _t = thread::spawn(move || avatar_socket.start_polling_with_duration(None));
    let sk = stereokit::Settings::default().display_preference(Flatscreen).init().unwrap();
    let devices = Microphone::device_count();
    for i in 0..devices {
        println!("{}, {}", Microphone::device_name(i), i);
    }
    let mic = Microphone::new(1);
    mic.start();
    let mut locomotion_tracker = LocomotionTracker::new(1.0, 1.0, 1.0);
    let sound = mic.get_stream().unwrap();
    let mut sounds: HashMap<SocketAddr, (SoundStream, SoundInstance)> = HashMap::new();
    let mut avatars: HashMap<SocketAddr, stereokit_vrm::VrmAvatar> = HashMap::new();
    let mut sample_num = 0;
    let mut this_model = stereokit_vrm::VrmAvatar::load_from_file(&sk, "Malek.vrm", &Shader::default(&sk)).unwrap();
    sk.run(|sk| {
        locomotion_tracker.locomotion_update(sk);
        //avatar stuff
        {
            this_model.update_ik(sk);
            this_model.draw(sk, &Pose::IDENTITY);
            let stuff_and_things = this_model.get_nodes_and_poses();
            let bytes = bincode::serialize(&AvatarInfo::new(client_addr, stuff_and_things)).unwrap();
            let packet = Packet::reliable_unordered(avatar_server_addr, bytes);
            avatar_tx.send(packet).unwrap();
            match avatar_rx.try_recv() {
                Ok(SocketEvent::Packet(packet)) => {
                    let avatar_info: AvatarInfo = bincode::deserialize(packet.payload()).unwrap();
                    if !avatars.contains_key(&avatar_info.address) {
                        avatars.insert(avatar_info.address, VrmAvatar::load_from_file(sk, "Malek.vrm", &Shader::default(sk)).unwrap());
                    }
                    avatars.get_mut(&avatar_info.address)
                        .unwrap().set_nodes_and_poses(avatar_info.nodes_and_poses);
                }
                _ => {}
            }
        }
        sample_num += 1;
        if sample_num > 3 {
            sample_num = 0;
            let len = sound.unread_samples();
            //println!("len: {}", len);
            let mut samples_with_pos = SamplesWithPos::new(sk.input_head().position.into(), client_addr, len);
            //println!("{:#?}", samples_with_pos);
            sound.read_samples(samples_with_pos.samples.as_mut_slice());
            let bytes = bincode::serialize(&samples_with_pos).unwrap();
            let packet = Packet::reliable_unordered(server_addr, bytes);
            tx.send(packet).unwrap();
        }
        match rx.try_recv() {
            Ok(SocketEvent::Packet(packet)) => {
                //println!("recieved packet");
                let samples_with_pos: SamplesWithPos = bincode::deserialize(packet.payload()).unwrap();
                if sounds.contains_key(&samples_with_pos.client) {
                    let (stream, instance) = sounds.get(&samples_with_pos.client).unwrap();
                    stream.write_samples(samples_with_pos.samples.as_slice());
                    instance.set_position(samples_with_pos.pos);
                } else {
                    let sound_stream = SoundStream::create(20.0);
                    sound_stream.write_samples(samples_with_pos.samples.as_slice());
                    let instance = sound_stream.play_sound(samples_with_pos.pos, 2.0);
                    sounds.insert(samples_with_pos.client, (sound_stream, instance));
                }
            }
            _ => {}
        }
    }, |_| {});
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AvatarInfo {
    nodes_and_poses: Vec<(NodeId, Pose)>,
    address: SocketAddr,
}

impl AvatarInfo {
    pub fn new(address: SocketAddr, nodes_and_poses: Vec<(NodeId, Pose)>) -> Self {
        Self {
            nodes_and_poses,
            address,
        }
    }
}

pub fn client() -> Result<()> {
    laminar_version();
    return Ok(());
    let server_address = "74.207.246.102:8888";
    let config = Default::default();

    // Create a client object
    let mut client = uflow::client::Client::connect(server_address, config).unwrap();

    println!("Type a message and press Enter to send. Send `Bye!` to quit.");

    let stdin = stdin();
    let mut s_buffer = String::new();

    let sk = stereokit::Settings::default().display_preference(DisplayMode::Flatscreen).init().unwrap();
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
        if sample_counter >= 2 {
             sample_counter = 0;
            let len = sound.unread_samples();
            //println!("len: {}", len);
            let mut samples_with_pos = SamplesWithPos::new(sk.input_head().position.into(), client.remote_address(), len);
            //println!("{:#?}", samples_with_pos);
            sound.read_samples(samples_with_pos.samples.as_mut_slice());
            let bytes = bincode::serialize(&samples_with_pos).unwrap();
            //println!("len: {}", bytes.len());
            //println!("bytes: {:?}", bytes);
            //println!("sending bytes: {:#?}", bytes);
            client.send(bytes.into_boxed_slice(), client.remote_address().port() as usize % 8, SendMode::Unreliable);
        }
        println!("{}", client.send_buffer_size());
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
                        //instance.set_position(samples_with_pos.pos);
                    } else {
                        let sound_stream = SoundStream::create(20.0);
                        sound_stream.write_samples(samples_with_pos.samples.as_slice());
                        let instance = sound_stream.play_sound(samples_with_pos.pos, 2.0);
                        sounds.insert(samples_with_pos.client, (sound_stream, instance));
                    }
                }
            }
        }
        client.flush();
        std::thread::sleep(Duration::from_millis(10));
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