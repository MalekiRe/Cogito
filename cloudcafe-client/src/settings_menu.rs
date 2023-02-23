use std::cell::RefCell;
use std::rc::Rc;
use stereokit::lifecycle::StereoKitDraw;
use stereokit::microphone::Microphone;
use stereokit::pose::Pose;
use stereokit::text::TextAlign;
use stereokit::ui::{MoveType, window, WindowType};
use crate::resources::{AppSettings, Resources};

pub struct SettingsMenu {
    visible: bool,
    pose: Pose,
    resources: Resources,
    microphones: Vec<Microphone>,
}
impl SettingsMenu {
    pub fn from(resources: &Resources) -> Self {
        Self::new(resources.clone())
    }
    fn new(resources: Resources) -> Self {
        Self {
            visible: true,
            pose: Pose::IDENTITY,
            resources,
            microphones: vec![],
        }
    }
    pub fn exist(&mut self, sk: &StereoKitDraw) {
        if self.visible {
            self.update_mics();
            self.draw(sk);
        }
    }
    fn update_mics(&mut self) {
        self.microphones.clear();
        for mic_num in 0..Microphone::device_count() {
            self.microphones.push(Microphone::new(mic_num))
        }
    }
    fn draw(&mut self, sk: &StereoKitDraw) {
        let height = self.microphones.len() as f32 * 0.05;
        let mut longest_str = 0;
        for mic in &self.microphones {
            if mic.get_name().len() > longest_str {
                longest_str = mic.get_name().len();
            }
        }
        let width = longest_str as f32 * 0.009;
        window(sk, "Settings", &mut self.pose, [width, height].into(), WindowType::WindowNormal, MoveType::MoveExact, |ui| {
            for (i, mic) in self.microphones.iter().enumerate() {
                if i == self.resources.0.borrow().app_settings.selected_mic as usize {
                    ui.toggle(mic.get_name(), &mut true);
                } else {
                    if ui.toggle(mic.get_name(), &mut false) {
                        self.resources.0.borrow_mut().app_settings.selected_mic = i as u32;
                        self.resources.update_settings_file();
                    }
                }
            }
            ui.text("Attribution for the world model to Isuiza under CC By 4.0", TextAlign::Center);
        });
    }
}
