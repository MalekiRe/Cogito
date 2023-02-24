use std::io::Read;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::ops::Deref;
use std::sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::thread::{JoinHandle, Thread};
use bincode::serialize;
use glam::{Mat4, Quat, Vec3};
use stereokit::color_named::WHITE;
use stereokit::font::Font;
use stereokit::lifecycle::{StereoKitContext, StereoKitDraw};
use stereokit::model::Model;
use stereokit::pose::Pose;
use stereokit::render::RenderLayer;
use stereokit::text::TextStyle;
use stereokit::ui::{MoveType, window, WindowContext, WindowType};
use stereokit::ui::layout::{layout_cut, Side};
use cloudcafe_common::ports::{SERVER_STATUS_PORT, ServerInfo};

pub enum MainMenuMsg {
    ConnectToServer(ServerInfo),
}

pub struct MainMenu {
    pose: Pose,
    model: Model,
    possible_server_ips: Vec<Ipv4Addr>,
    server_infos: Arc<Mutex<Vec<ServerInfo>>>,
    server_infos_thread: Option<JoinHandle<()>>,
    connected_server: Option<ServerInfo>,
}
impl MainMenu {
    pub fn new(sk: &impl StereoKitContext) -> Self {
        let monument_model = Model::from_file(sk, "statue.glb", None).unwrap();
        Self {
            pose: Pose::new([0.0, 1.8, -0.50],
                            Quat::IDENTITY),
            model: monument_model,

            possible_server_ips: vec![Ipv4Addr::new(127, 0, 0, 1)],
            server_infos: Arc::new(Mutex::new(vec![])),
            server_infos_thread: None,
            connected_server: None,
        }
    }
    pub fn exist(&mut self, sk: &StereoKitDraw) -> Option<MainMenuMsg> {
        self.update_server_infos_thread();
        self.model.draw(sk, Mat4::from_scale_rotation_translation(Vec3::new(12.0, 16.0, 12.0), Quat::IDENTITY, Vec3::new(0.0, 2.0, 0.0)).into(), WHITE, RenderLayer::Layer1);
        if let Some(server_info) =  self.draw(sk) {
            return Some(MainMenuMsg::ConnectToServer(server_info));
        }
        None
    }
    fn refresh_server_infos(&mut self) {
        if self.server_infos_thread.is_none() {
            let server_infos = self.server_infos.clone();
            let server_ips = self.possible_server_ips.clone();
            self.server_infos_thread = Some(thread::spawn(move || {
                loop {
                    match server_infos.lock() {
                        Ok(mut server_infos) => {
                            server_infos.clear();
                            break;
                        }
                        Err(_) => {}
                    }
                }
                for ip in server_ips {
                    match TcpStream::connect(SocketAddrV4::new(ip, SERVER_STATUS_PORT)) {
                        Ok(mut tcp_stream) => {
                            let mut buff = Vec::new();
                            tcp_stream.read_to_end(&mut buff).unwrap();
                            loop {
                                match server_infos.lock() {
                                    Ok(mut server_infos) => {
                                        server_infos.push(bincode::deserialize(buff.as_slice()).unwrap());
                                        break;
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }));
        }
    }
    fn update_server_infos_thread(&mut self) {
        match &self.server_infos_thread {
            None => {}
            Some(thread) => {
                if thread.is_finished() {
                    self.server_infos_thread.take().unwrap().join().unwrap();
                }
            }
        }
    }
    fn draw(&mut self, sk: &StereoKitDraw) -> Option<ServerInfo> {
        let mut window_pose = self.pose;
        let mut ret_val = None;
        window(&sk, "", &mut window_pose, [0.7, 1.1].into(), WindowType::WindowBody, MoveType::MoveNone, |ui| {
            ret_val = self.draw_server_info(sk, ui);
            layout_cut(ui, Side::Right, 0.08, |layout| {
                layout.ui(|ui| {
                    if ui.button("refresh") {
                        self.refresh_server_infos();
                    }
                });
            });
        });
        self.pose = window_pose;
        ret_val
    }
    fn draw_server_info(&mut self, sk: &impl StereoKitContext, ui: &WindowContext) -> Option<ServerInfo> {
        loop {
            match &self.server_infos.lock() {
                Ok(server_infos) => {
                    for server_info in server_infos.iter() {
                        let title = TextStyle::new(sk, Font::default(sk), 0.02, stereokit::color_named::MOCCASIN);
                        let ip_style = TextStyle::new(sk, Font::default(sk), 0.01, stereokit::color_named::AQUAMARINE);
                        let spacing = 0.02;
                        let mut possible_return_val = None;
                        ui.text_style(title, |ui| {
                            if ui.button(server_info.name.as_str()) {
                                possible_return_val = Some(server_info.clone());
                            }
                        });
                        if let Some(return_val) = possible_return_val {
                            return Some(return_val);
                        }
                        ui.label(" ", false); ui.sameline(); ui.space(spacing);
                        ui.label("IP Address:", false); ui.sameline();
                        ui.text_style(ip_style.clone(), |ui| {
                            ui.label(server_info.ip.to_string().as_str(), false);
                        });
                        ui.label(" ", false); ui.sameline(); ui.space(spacing);
                        ui.label("Users:          ", false); ui.sameline();
                        ui.text_style(ip_style.clone(), |ui|  ui.label(server_info.player_count.to_string().as_str(), false));
                        ui.label(" ", false); ui.sameline(); ui.space(spacing);
                        ui.label("Map:            ", false); ui.sameline();
                        ui.text_style(ip_style.clone(), |ui|  ui.label(format!("{:?}", server_info.map).as_str(), false));
                        ui.space(0.1);
                    }
                    break;
                }
                Err(_) => {}
            }
        }
        None
    }
}