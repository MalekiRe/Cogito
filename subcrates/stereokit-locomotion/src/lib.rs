use std::cmp::min;
use std::ops::AddAssign;
use glam::{EulerRot, Mat4, Quat, Vec2, Vec3};
use glam::EulerRot::XYZ;
use stereokit::hierarchy::hierarchy;
use stereokit::input::Handed;
use stereokit::lifecycle::{StereoKitContext, StereoKitDraw};
use stereokit::pose::Pose;
use stereokit::render::Camera;
use stereokit::values::{MQuat, MVec3};

#[cfg(feature = "physics")]
use stereokit::physics::Solid;

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
        let mut locomotion_tracker = LocomotionTracker::new(0.1, 1.5, 1.0);
        sk.run(|sk| {
            locomotion_tracker.analogue_controls(sk);
            //locomotion_tracker.locomotion_update(sk);
            let mat = Material::copy_from_id(sk, DEFAULT_ID_MATERIAL).unwrap();
            let mesh = Mesh::gen_cube(sk, Vec3::new(0.1, 0.1, 0.1), 1).unwrap();
            let model = Model::from_mesh(sk, &mesh, &mat).unwrap();
            model.draw(sk, Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), Quat::IDENTITY, Vec3::default()).into(), NAVY, RenderLayer::Layer0);
        }, |_|{});
        Ok(())
    }
}
pub fn rotate_player(angle: Quat) {

}
pub struct LocomotionTracker {
    /// the center of your play area at eye height.
    pub stage_pose: Pose,
    pub flying_enabled: bool,
    pub toggle: f32,
    pub rotation_sensitivity: f32,
    pub rotation_speed_multiplier: f32,
    pub position_speed_multiplier: f32,
    pub controller_rotation: f32,
    #[cfg(feature = "physics")]
    pub solid: Option<Solid>,
}
impl LocomotionTracker {
    pub fn new(rotation_sensitivity: f32, rotation_speed_multiplier: f32, position_speed_multiplier: f32) -> Self {
        Self {
            stage_pose: Pose::IDENTITY,
            flying_enabled: false,
            toggle: 1.0,
            rotation_sensitivity,
            rotation_speed_multiplier,
            position_speed_multiplier,
            controller_rotation: 0.0,
            #[cfg(feature="physics")]
            solid: None,
        }
    }
    #[cfg(feature="physics")]
    pub fn new_physics(rotation_sensitivity: f32, rotation_speed_multiplier: f32, position_speed_multiplier: f32, solid: Solid) -> Self {
        Self {
            stage_pose: Pose::IDENTITY,
            flying_enabled: false,
            toggle: 1.0,
            rotation_sensitivity,
            rotation_speed_multiplier,
            position_speed_multiplier,
            controller_rotation: 0.0,
            #[cfg(feature="physics")]
            solid: Some(solid),
        }
    }
}
impl LocomotionTracker {
    fn get_pose(&self, sk: &impl StereoKitContext) -> Pose {
        #[cfg(feature = "physics")]
        {
            match &self.solid {
                None => {
                    self.stage_pose
                }
                Some(solid) => {
                    solid.get_pose(sk)
                }
            }
        }
        #[cfg(not(feature = "physics"))]
        self.stage_pose
    }
    fn set_pose(&mut self, sk: &impl StereoKitContext, pose: Pose) {
        #[cfg(feature = "physics")]
        {
            match &mut self.solid {
                None => {
                    self.stage_pose = pose;
                }
                Some(solid) => {
                    solid.teleport_to(pose);
                }
            }
        };
        #[cfg(not(feature = "physics"))]
        {
            self.stage_pose = pose;
        }
    }
    fn move_to_pose(&mut self, sk: &impl StereoKitContext, pose: Pose) {
        #[cfg(feature = "physics")]
        {
            match &mut self.solid {
                None => {
                    self.stage_pose = pose;
                }
                Some(solid) => {
                    solid.move_to(pose);
                }
            }
        };
        #[cfg(not(feature = "physics"))]
        {
            self.stage_pose = pose;
        };
    }
    pub fn rotate_player(&mut self, sk: &impl StereoKitContext, angle: impl Into<MQuat>) {
        let angle = angle.into();
        let mut temp_pose = self.get_pose(sk);
        hierarchy(Mat4::from_translation(sk.input_head().position.into()).into(), |h| {
            temp_pose = h.to_local_pose(temp_pose);
            hierarchy(Mat4::from_quat(angle.into()).into(), |h| {
                temp_pose = h.to_world_pose(temp_pose);
            });
        });
        self.set_pose(sk, temp_pose);
        self.sync_camera_stage_pose(sk);
    }
    pub fn sync_camera_stage_pose(&mut self, sk: &impl StereoKitContext) {
        let mut pose = self.get_pose(sk);
        if pose.position.y > 0.8 {
            pose.position.y -= 0.05;
        }
        if pose.position.y < 0.5 {
            pose.position.y += 0.05;
        }
        self.set_pose(sk, Pose::new(Vec3::new(pose.position.x, pose.position.y, pose.position.z), pose.orientation));
        Camera::set_root(sk, pose.as_matrix());
    }
    pub fn set_player_position(&mut self, sk: &impl StereoKitContext, new_position: impl Into<MVec3>) {
        let new_position = Vec3::from(new_position.into());
        let player_position_in_stage = Mat4::from(self.stage_pose.as_matrix()).inverse().transform_vector3(sk.input_head().position.into());
        self.stage_pose.position = MVec3::from(new_position - Mat4::from_quat(self.stage_pose.orientation.into()).transform_vector3(player_position_in_stage));
        self.sync_camera_stage_pose(sk);
    }
    pub fn move_player_position(&mut self, sk: &impl StereoKitContext, locomotion_direction: impl Into<MVec3>) {
        let mut locomotion_direction = Vec3::from(locomotion_direction.into());
        if !self.flying_enabled {
            locomotion_direction *= Vec3::new(1.0, 0.0, 1.0);
        }
        let speed = self.position_speed_multiplier * sk.time_elapsedf() * 180.0;
        locomotion_direction *= Vec3::new(speed, speed, speed);
        self.move_to_pose(sk, Pose::new(MVec3::from(Vec3::from(self.get_pose(sk).position) + locomotion_direction), self.get_pose(sk).orientation));
        self.sync_camera_stage_pose(sk);
    }
    pub fn analogue_controls(&mut self, sk: &impl StereoKitContext) {
        let abs_rotation = self.controller_rotation.abs();
        if abs_rotation > self.rotation_sensitivity {
            let speed = abs_rotation - self.rotation_sensitivity;
            let speed = 180.0 * speed * self.controller_rotation.signum() * sk.time_elapsedf() * self.rotation_speed_multiplier;
            self.rotate_player(sk, quat_from_angles(0.0, speed, 0.0));
        }
        let move_stick = sk.input_controller(Handed::Left).stick;
        if move_stick.x.abs() > self.rotation_sensitivity || move_stick.y.abs() > self.rotation_sensitivity {
            let move_vec = Vec3::new(-move_stick.x / 90.0, 0.0, -move_stick.y / 90.0);
            let locomotion_direction = Mat4::from_quat(sk.input_head().orientation.into()).transform_vector3(move_vec);
            self.move_player_position(sk, locomotion_direction);
        }
        self.controller_rotation = sk.input_controller(Handed::Right).stick.x;
        self.sync_camera_stage_pose(sk);
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