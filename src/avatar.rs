use std::fs::File;
use std::{fs, io};
use std::path::Path;
use stereokit::lifecycle::{StereoKitContext, StereoKitDraw};
use stereokit::model::{Model, NodeId};
use color_eyre::Result;
use glam::{Mat4, Vec3};
use serde_json::Value;
use stereokit::color_named::WHITE;
use stereokit::pose::Pose;
use stereokit::render::RenderLayer;

pub struct Avatar {
    model: Model,
    avatar_nodes: AvatarNodes,
}


pub enum AvatarNode {
    Torso(Torso),
    Head(Head),
    Leg(Leg),
    Arm(Arm),
    Finger(Finger),
}
pub enum Torso {
    Hips,
    Spin,
    Chest,
    UpperChest,
    Neck,
}
pub enum Head {
    Head,
    LeftEye,
    RightEye,
    Jaw,
}
pub enum Leg {

}
pub enum Arm {
    LeftShoulder,
    LeftUpperArm,
    LeftLowerArm,
    LeftHand,
    RightShoulder,
    RightUpperArm,
    RightLowerArm,
    RightHand,
}
pub enum Finger {

}

pub struct AvatarNodes {
    head: NodeId,
    hips: NodeId,
    left: SidedNodes,
    right: SidedNodes,
}

impl AvatarNodes {
    pub fn from_model(sk: &impl StereoKitContext, model: &Model) -> Self {
        let head = model.node_find("J_Bip_C_Head").unwrap();
        let hips = model.node_find("J_Bip_C_Hips").unwrap();
        Self {
            head,
            hips,
            left: SidedNodes::from_model(sk, model),
            right: SidedNodes::from_model(sk, model)
        }
    }
}

pub struct SidedNodes {

}

impl SidedNodes {
    pub fn from_model(sk: &impl StereoKitContext, model: &Model) -> Self {
        Self {}
    }
}

impl Avatar {
    pub fn from_file(sk: &impl StereoKitContext, file_path: impl AsRef<Path>) -> Result<Self> {
        let model = Model::from_file(sk, file_path.as_ref(), None)?;
        let avatar_nodes = AvatarNodes::from_model(sk, &model);
        let json = import_json(file_path.as_ref().to_str().unwrap());
        println!("{:#?}", json);
        Ok(Self {
            model,
            avatar_nodes,
        })
    }
    pub fn draw(&self, sk: &StereoKitDraw, pose: Pose) {
        let mat = Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), pose.orientation.into(), pose.position.into());
        self.model.draw(sk, mat.into(), WHITE, RenderLayer::Layer0);
    }
}

pub fn import_json(file: &str) {
    let bytes = std::fs::read(&file).unwrap();
    let thing: goth_gltf::Gltf<goth_gltf::default_extensions::Extensions> = goth_gltf::Gltf::from_bytes(&bytes).unwrap().0;
    println!("{:#?}", thing);
}