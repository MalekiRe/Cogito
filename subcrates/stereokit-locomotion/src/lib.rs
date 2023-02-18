use std::ops::AddAssign;
use glam::{EulerRot, Mat4, Quat, Vec2, Vec3};
use glam::EulerRot::XYZ;
use stereokit::input::Handed;
use stereokit::lifecycle::StereoKitContext;
use stereokit::render::Camera;


#[cfg(test)]
mod test {
    use stereokit::Settings;
    use color_eyre::Result;
    use glam::{Mat4, Quat, Vec3};
    use stereokit::color_named::NAVY;
    use stereokit::material::{DEFAULT_ID_MATERIAL, Material};
    use stereokit::mesh::Mesh;
    use stereokit::model::Model;
    use stereokit::render::RenderLayer;
    use crate::{LocomotionTracker};

    #[test]
    pub fn run() -> Result<()> {
        let sk = Settings::default().init()?;
        let mut locomotion_tracker = LocomotionTracker::new(0.2, 1.5, 1.0);
        sk.run(|sk| {
            locomotion_tracker.locomotion_update(sk);
            let mat = Material::copy_from_id(sk, DEFAULT_ID_MATERIAL).unwrap();
            let mesh = Mesh::gen_cube(sk, Vec3::new(0.1, 0.1, 0.1), 1).unwrap();
            let model = Model::from_mesh(sk, &mesh, &mat).unwrap();
            model.draw(sk, Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), Quat::IDENTITY, Vec3::default()).into(), NAVY, RenderLayer::Layer0);
        }, |_|{});
        Ok(())
    }
}

pub struct LocomotionTracker {
    pub toggle: f32,
    pub rotation_sensitivity: f32,
    pub rotation_speed_multiplier: f32,
    pub position_speed_multiplier: f32,
}
impl LocomotionTracker {
    pub fn new(rotation_sensitivity: f32, rotation_speed_multiplier: f32, position_speed_multiplier: f32) -> Self {
        Self {
            toggle: 1.0,
            rotation_sensitivity,
            rotation_speed_multiplier,
            position_speed_multiplier,
        }
    }
}
impl LocomotionTracker {
    pub fn locomotion_update(&mut self, sk: &impl StereoKitContext) {
        let right_controller = sk.input_controller(Handed::Right);
        let rot_x = right_controller.stick.x;
        let (scale, rot, mut position) = Mat4::from(Camera::get_root(sk)).to_scale_rotation_translation();
        let (x, mut y, z) = angles_from_quat(rot);
        if rot_x >= self.rotation_sensitivity {
            y += 1.0 * self.toggle * self.rotation_speed_multiplier;
        }
        if rot_x <= -self.rotation_sensitivity {
            y -= 1.0 * self.toggle * self.rotation_speed_multiplier;
        }
        if y > 90.0 || y <= -90.0 {
            self.toggle *= -1.0;
        }

        let left_controller = sk.input_controller(Handed::Left);
        let stick_x = left_controller.stick.x;
        let stick_y = left_controller.stick.y;
        let mut r#move = Vec2::default();
        if stick_x > 0.1 || stick_x < -0.1 {
            r#move.x -= self.position_speed_multiplier * stick_x/200.0;
        }
        if stick_y > 0.1 || stick_y < -0.1 {
            r#move.y -= self.position_speed_multiplier * stick_y/200.0;
        }

        let m = Mat4::from_rotation_translation(quat_from_angles(x, y, z), Vec3::default());
        position.add_assign(m.transform_point3(Vec3::new(r#move.x, 0.0, r#move.y)));
        let matrix = Mat4::from_scale_rotation_translation(
            scale, quat_from_angles(x, y, z), position
        );
        Camera::set_root(sk, matrix);
    }
}
pub fn quat_from_angles(x: f32, y: f32, z: f32) -> Quat {
    Quat::from_euler(
        EulerRot::XYZ,
        x.to_radians(),
        y.to_radians(),
        z.to_radians(),
    )
}
pub fn quat_from_vec(vec: Vec3) -> Quat {
    quat_from_angles(vec.x, vec.y, vec.z)
}
pub fn angles_from_quat(quat: Quat) -> (f32, f32, f32) {
    let radians_version = quat.to_euler(XYZ);
    (
        radians_version.0.to_degrees(),
        radians_version.1.to_degrees(),
        radians_version.2.to_degrees(),
    )
}
pub fn angles_from_quat_vec(quat: Quat) -> Vec3 {
    let tuple = angles_from_quat(quat);
    Vec3::new(tuple.0, tuple.1, tuple.2)
}