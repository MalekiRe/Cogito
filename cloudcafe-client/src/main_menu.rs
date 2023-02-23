use glam::{Mat4, Quat, Vec3};
use stereokit::color_named::WHITE;
use stereokit::lifecycle::{StereoKitContext, StereoKitDraw};
use stereokit::model::Model;
use stereokit::pose::Pose;
use stereokit::render::RenderLayer;
use stereokit::ui::{MoveType, window, WindowType};

pub struct MainMenu {
    pose: Pose,
    model: Model,
}
impl MainMenu {
    pub fn new(sk: &impl StereoKitContext) -> Self {
        let monument_model = Model::from_file(sk, "statue.glb", None).unwrap();
        Self { pose: Pose::new([0.0, 0.2, -0.20], Quat::IDENTITY), model: monument_model }
    }
    pub fn exist(&mut self, sk: &StereoKitDraw) {
        self.model.draw(sk, Mat4::from_scale_rotation_translation(Vec3::new(5.0, 6.0, 5.0), Quat::IDENTITY,Vec3::new(0.0, 0.42, 0.0)).into(), WHITE, RenderLayer::Layer1);
        self.draw(sk);
    }
    fn draw(&mut self, sk: &StereoKitDraw) {
        window(&sk, "", &mut self.pose, [0.2, 0.2].into(), WindowType::WindowBody, MoveType::MoveNone, |ui| {
            ui.button("hi");
        });
    }
}