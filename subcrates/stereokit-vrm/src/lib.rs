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
use stereokit::material::{Cull, DEFAULT_ID_MATERIAL, Material};
use stereokit::mesh::Mesh;
use stereokit::model::{Model, NodeId};
use stereokit::pose::Pose;
use stereokit::render::{Rect, RenderClear, RenderLayer, StereoKitRender};
use stereokit::shader::Shader;
use stereokit::texture::{Texture, TextureFormat, TextureType};
use stereokit::values::MMatrix;
use crate::bones::Skeleton;
use crate::ik::{Ik, quat_from_angles};

pub type VrmGltf = Gltf<goth_gltf::default_extensions::Extensions>;


pub struct VrmAvatar {
    model: Model,
    pub skeleton: Skeleton,
    ik: Ik,
    gltf: VrmGltf,
}

pub static mut VRM_SHADER: Option<Shader> = None;

impl VrmAvatar {
    pub fn get_shader(sk: &impl StereoKitContext) -> &'static Shader {
        unsafe {
            VRM_SHADER.as_ref().unwrap()
        }
    }
    pub fn load_from_file(sk: &impl StereoKitContext, file: impl AsRef<Path>, shader: &Shader) -> Result<Self> {
        let model = Model::from_file(sk, file.as_ref(), Some(shader))?.clone();
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
        self.model.draw(sk, matrix, WHITE, RenderLayer::LayerAll);
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
    //let shader = Shader::from_file(&sk, "malek.sks").unwrap();
    let shader = Shader::default(&sk);
    let mut avatar = VrmAvatar::load_from_file(&sk, "Malek.vrm", &shader).unwrap();

//    let mesh = Mesh::gen_cube(&sk, [0.3, 0.3, 0.3], 1).unwrap();
//    let mut material = Material::create(&sk, &Shader::default(&sk)).unwrap();

    //let texture = Texture::create(&sk, TextureType::RenderTarget, TextureFormat::No).unwrap();
//    texture.set_size(512, 512);
//    texture.add_zbuffer(TextureFormat::Depth16);
    //material.set_texture(&sk, "diffuse", &texture).unwrap();
    //material.set_cull(&sk, Cull::Front);
    //let model = Model::from_mesh(&sk, &mesh, &material).unwrap();
    sk.run(|sk| {
        let pos = Mat4::from_scale_rotation_translation(Vec3::new(0.1, 0.1, 0.1), quat_from_angles(0.0, 90.0, 20.0), Vec3::new(1.0, 1.0, 1.0));
        /*sk.render_to(&texture, pos, Mat4::orthographic_rh_gl(-1.0, 1.0, -1.0, 1.0, 0.01, 0.1), RenderLayer::all().difference(RenderLayer::Layer0), RenderClear::None, Rect{
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        });*/
        //model.draw(sk, pos.into(), WHITE, RenderLayer::Layer0);
        avatar.draw(sk, &Pose::IDENTITY);
        avatar.update_ik(sk);
        //println!("{:?}", avatar.get_nodes_and_poses());
        //model.draw(sk, Mat4::default().into(), WHITE, RenderLayer::Layer0);
    }, |_| {});
}