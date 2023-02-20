mod bones;
mod ik;

use std::fs::File;
use std::path::Path;
use stereokit::lifecycle::{StereoKitContext, StereoKitDraw};
use color_eyre::{Report, Result};
use glam::{Mat4, Vec3};
use goth_gltf::Gltf;
use simple_error::{SimpleError, SimpleResult};
use stereokit::color_named::WHITE;
use stereokit::model::{Model, NodeId};
use stereokit::pose::Pose;
use stereokit::render::RenderLayer;
use stereokit::shader::Shader;
use stereokit::values::MMatrix;
use crate::bones::Skeleton;
use crate::ik::{Ik, quat_from_angles};

pub type VrmGltf = Gltf<goth_gltf::default_extensions::Extensions>;


pub struct VrmAvatar {
    model: Model,
    skeleton: Skeleton,
    ik: Ik,
    gltf: VrmGltf,
    number: u32,
    n2: u32,
    r1: f32,
    r2: f32,
    x: f32,
    y: f32,
    z: f32,
}

pub static mut VRM_SHADER: Option<Shader> = None;

impl VrmAvatar {
    pub fn get_shader(sk: &impl StereoKitContext) -> &'static Shader {
        unsafe {
            VRM_SHADER.as_ref().unwrap()
        }
    }
    pub fn load_from_file(sk: &impl StereoKitContext, file: impl AsRef<Path>, shader: &Shader) -> Result<Self> {
        let model = Model::from_file(sk, file.as_ref(), Some(shader))?;
        let bytes = std::fs::read(&file).unwrap();
        let (gltf, b): (
            goth_gltf::Gltf<goth_gltf::default_extensions::Extensions>,
            _,
        ) = goth_gltf::Gltf::from_bytes(&bytes)?;
        if gltf.extensions.vrm.is_none() {
            return Err(Report::from(SimpleError::new("Vrm model doesn't have the VRM extension")));
        }
        let skeleton = Skeleton::new(&gltf)?;
        let ik = Ik::new(&gltf, &model, &skeleton);
        Ok(VrmAvatar {
            model,
            skeleton,
            ik,
            gltf,
            number: 0,
            n2: 0,
            r1: 0.0,
            r2: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })

    }
    pub fn l_n_pos(&self, node: NodeId) -> Vec3 {
        Mat4::from(self.model.node_get_transform_local(node)).to_scale_rotation_translation().2
    }
    pub fn draw(&self, sk: &StereoKitDraw, pose: &Pose) {
        let matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(1.0, 1.0, 1.0),
            pose.orientation.into(),
            pose.position.into()).into();
        self.model.draw(sk, matrix, WHITE, RenderLayer::Layer0);
    }
    pub fn _get_node(gltf: &VrmGltf, node: &str) -> SimpleResult<NodeId> {
        for b in &gltf.extensions.vrm.clone().unwrap().humanoid.human_bones {
            if b.bone == node {
                return NodeId::try_from(b.node as i32).ok_or(SimpleError::new(format!("missing bone: {}", node)));
            }
        }
        return Err(SimpleError::new(format!("missing bone: {}", node)));
    }
    pub fn get_node(&self, node: &str) -> SimpleResult<NodeId> {
        Self::_get_node(&self.gltf, node)
    }
    pub fn pose_node_local(&mut self, node: NodeId, pose: Pose) {
        self.model.node_set_transform_local(node, pose.as_matrix());
    }
    pub fn pose_node_model(&mut self, node: NodeId, pose: Pose) {
        //let rotation: Mat4 = self.model.node_get_transform_model(node).into();
        //let new_matrix = Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), rotation.to_scale_rotation_translation().1, pose.position.into());
        self.model.node_set_transform_model(node, pose.as_matrix());
    }
    pub fn transform_node_model(&mut self, node: NodeId, matrix: Mat4) {
        self.model.node_set_transform_model(node, matrix.into());
    }
    pub fn transform_node_local(&mut self, node: NodeId, matrix: Mat4) {
        self.model.node_set_transform_local(node, matrix.into());
    }
}

#[cfg(test)]
mod test {
    use color_eyre::Result;
    use stereokit::pose::Pose;
    use stereokit::StereoKit;
    use stereokit::Settings;
    use stereokit::input::{Handed, StereoKitInput};
    use crate::VrmAvatar;

    #[test]
    pub fn run() -> Result<()> {
        let sk: StereoKit = Settings::default().init()?;
        let mut avatar = VrmAvatar::load_from_file(&sk, "../../Malek.vrm")?;
        sk.run(|sk| {
            avatar.draw(sk, &Pose::IDENTITY);
            avatar.update_ik(sk);
        }, |_| {});
        Ok(())
    }
}

pub fn main() {
    let sk: stereokit::StereoKit = stereokit::Settings::default().init().unwrap();
    let shader = Shader::from_file(&sk, "malek.sks").unwrap();
    let mut avatar = VrmAvatar::load_from_file(&sk, "Malek.vrm", &shader).unwrap();

    sk.run(|sk| {
        avatar.draw(sk, &Pose::IDENTITY);
        avatar.update_ik(sk);
    }, |_| {});
}