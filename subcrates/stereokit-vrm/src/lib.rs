mod bones;
mod ik;

use std::path::Path;
use stereokit::lifecycle::{StereoKitContext, StereoKitDraw};
use color_eyre::{Report, Result};
use glam::{Mat4, Vec3};
use goth_gltf::Gltf;
use simple_error::SimpleError;
use stereokit::color_named::WHITE;
use stereokit::model::{Model, NodeId};
use stereokit::pose::Pose;
use stereokit::render::RenderLayer;
use crate::bones::Skeleton;

pub type VrmGltf = Gltf<goth_gltf::default_extensions::Extensions>;


pub struct VrmAvatar {
    model: Model,
    skeleton: Skeleton,
    gltf: VrmGltf,
}

impl VrmAvatar {
    pub fn load_from_file(sk: &impl StereoKitContext, file: impl AsRef<Path>) -> Result<Self> {
        let model = Model::from_file(sk, file.as_ref(), None)?;
        let bytes = std::fs::read(&file).unwrap();
        let (gltf, b): (
            goth_gltf::Gltf<goth_gltf::default_extensions::Extensions>,
            _,
        ) = goth_gltf::Gltf::from_bytes(&bytes)?;
        if gltf.extensions.vrm.is_none() {
            return Err(Report::from(SimpleError::new("Vrm model doesn't have the VRM extension")));
        }
        let skeleton = Skeleton::new(&gltf).ok_or(SimpleError::new("Vrm model doesn't have all supported bones"))?;
        Ok(VrmAvatar {
            model,
            skeleton,
            gltf,
        })

    }
    pub fn draw(&self, sk: &StereoKitDraw, pose: &Pose) {
        let matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(1.0, 1.0, 1.0),
            pose.orientation.into(),
            pose.position.into()).into();
        self.model.draw(sk, matrix, WHITE, RenderLayer::Layer0);
    }
    pub fn _get_node(gltf: &VrmGltf, node: &str) -> Option<NodeId> {
        for b in &gltf.extensions.vrm.clone().unwrap().humanoid.human_bones {
            if b.bone == node {
                return NodeId::try_from(b.node as i32);
            }
        }
        return None;
    }
    pub fn get_node(&self, node: &str) -> Option<NodeId> {
        Self::_get_node(&self.gltf, node)
    }
    pub fn pose_node_local(&mut self, node: NodeId, pose: Pose) {
        self.model.node_set_transform_local(node, pose.as_matrix());
    }
    pub fn pose_node_model(&mut self, node: NodeId, pose: Pose) {
        let rotation: Mat4 = self.model.node_get_transform_model(node).into();
        let new_matrix = Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), rotation.to_scale_rotation_translation().1, pose.position.into());
        self.model.node_set_transform_model(node, new_matrix.into());
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