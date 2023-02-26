use std::ops::{Div, Neg, Sub};
use color_eyre::owo_colors::OwoColorize;
use glam::{EulerRot, Mat4, Quat, quat, Vec3};
use glam::EulerRot::XYZ;
use goth_gltf::extensions::CompressionFilter::Quaternion;
use goth_gltf::{Gltf, Node};
use stereokit::color_named::{BLUE, GREEN, ORANGE, RED, YELLOW};
use stereokit::input::{Handed, Joint, StereoKitInput};
use stereokit::lifecycle::StereoKitDraw;
use stereokit::material::DEFAULT_ID_MATERIAL;
use stereokit::model;
use stereokit::model::{Model, NodeId};
use stereokit::pose::Pose;
use stereokit::render::{RenderLayer, StereoKitRender};
use stereokit::ui::{ConfirmMethod, MoveType, window, WindowType};
use crate::{VrmAvatar, VrmGltf};
use crate::bones::{Skeleton};

impl VrmAvatar {
    pub fn update_ik(&mut self, sk: &StereoKitDraw) {

        //sk.input_hand_visible(Handed::Right, false);
        self.do_other_ik(sk);
        self.left_hand(sk);
        let right_hand = sk.input_hand(Handed::Right);




        let right_hand_xr_correction: Quat = quat_from_angles(0.0f32, 90.0f32, 0.0f32);
        // let right_hand_xr_correction: Quat = Quat::IDENTITY;

        // Don't ever do two euler rotations! More than one is bad luck and hard to think about! Multiply one single-axis rotation by another! 


        // I'm having a bit of trouble understanding why the first rotation needed to be z -90.
        // But basically the three phenomena are
        // * The way every bone is wrong (this one is odd - it's a rotation about the Y axis! I'd expect it to be about the X axis, if it's going from Blender to OpenXR. This might be a byproduct of VRM being weird.) Corresponds to right_hand_xr_correction.
        // * The fact that wrist bones don't connect in VRM and instead point directly to the side. Corresponds to fix_thumb_not_connect
        // * The fact that the thumb bones have the wrong "twist" and have their X axis point straight forward. Corresponds to fix_thumb_x_axis_wrong_direction
        let fix_thumb_not_connect =quat_from_angles(-0.0f32, -40.0f32, 0.0f32);
        let fix_thumb_x_axis_wrong_direction = quat_from_angles(-0.0f32, -0.0f32, -90.0f32);

        let right_hand_xr_correction_thumb: Quat = fix_thumb_x_axis_wrong_direction * right_hand_xr_correction * fix_thumb_not_connect; 
        // let right_hand_xr_correction_thumb: Quat = Quat::IDENTITY;


        let mut hand_pose = right_hand.wrist;
        let mut hand_orientation: Quat = hand_pose.orientation.into();

        hand_pose.orientation = (hand_orientation * right_hand_xr_correction).into();

        self.pose_node_model(self.skeleton.right_arm.hand, hand_pose);

        self.cats_meow(self.skeleton.right_finger.index.proximal, right_hand.fingers[1][1], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.index.intermediate, right_hand.fingers[1][2], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.index.distal, right_hand.fingers[1][3], right_hand_xr_correction);

        self.cats_meow(self.skeleton.right_finger.middle.proximal, right_hand.fingers[2][1], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.middle.intermediate, right_hand.fingers[2][2], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.middle.distal, right_hand.fingers[2][3], right_hand_xr_correction);

        self.cats_meow(self.skeleton.right_finger.ring.proximal, right_hand.fingers[3][1], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.ring.intermediate, right_hand.fingers[3][2], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.ring.distal, right_hand.fingers[3][3], right_hand_xr_correction);

        self.cats_meow(self.skeleton.right_finger.little.proximal, right_hand.fingers[4][1], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.little.intermediate, right_hand.fingers[4][2], right_hand_xr_correction);
        self.cats_meow(self.skeleton.right_finger.little.distal, right_hand.fingers[4][3], right_hand_xr_correction);



        self.cats_meow(self.skeleton.right_finger.thumb.proximal, right_hand.fingers[0][1], right_hand_xr_correction_thumb);
        self.cats_meow(self.skeleton.right_finger.thumb.intermediate, right_hand.fingers[0][2], right_hand_xr_correction_thumb);
        self.cats_meow(self.skeleton.right_finger.thumb.distal, right_hand.fingers[0][3], right_hand_xr_correction_thumb);


        let hand_size = Mat4::from_quat(Quat::from(right_hand.wrist.orientation)).transform_point3(Vec3::new(0.0, 0.0, -right_hand.size * 2.0));

        let lower_arm_pose = Pose::new(Vec3::from(hand_pose.position).sub(hand_size).lerp(Vec3::from(sk.input_head().position), 0.25), Quat::from(hand_pose.orientation).lerp(Quat::from(self.ik.lower_right_arm.orientation), 0.25));

        self.mrow(self.skeleton.right_arm.lower_arm, lower_arm_pose, quat_from_angles(180.0, 0.0, 0.0));

    }

    pub fn left_hand(&mut self, sk: &StereoKitDraw) {
        let left_hand = sk.input_hand(Handed::Left);




        let left_hand_xr_correction: Quat = quat_from_angles(0.0f32, 90.0f32, 0.0f32).neg();
        // let right_hand_xr_correction: Quat = Quat::IDENTITY;

        // Don't ever do two euler rotations! More than one is bad luck and hard to think about! Multiply one single-axis rotation by another!


        // I'm having a bit of trouble understanding why the first rotation needed to be z -90.
        // But basically the three phenomena are
        // * The way every bone is wrong (this one is odd - it's a rotation about the Y axis! I'd expect it to be about the X axis, if it's going from Blender to OpenXR. This might be a byproduct of VRM being weird.) Corresponds to right_hand_xr_correction.
        // * The fact that wrist bones don't connect in VRM and instead point directly to the side. Corresponds to fix_thumb_not_connect
        // * The fact that the thumb bones have the wrong "twist" and have their X axis point straight forward. Corresponds to fix_thumb_x_axis_wrong_direction
        let fix_thumb_not_connect =quat_from_angles(-0.0f32, -40.0f32, 0.0f32);
        let fix_thumb_x_axis_wrong_direction = quat_from_angles(-0.0f32, -0.0f32, -90.0f32);

        let left_hand_xr_correction_thumb: Quat = fix_thumb_x_axis_wrong_direction * left_hand_xr_correction * fix_thumb_not_connect;
        // let right_hand_xr_correction_thumb: Quat = Quat::IDENTITY;


        let mut hand_pose = left_hand.wrist;
        let mut hand_orientation: Quat = hand_pose.orientation.into();

        hand_pose.orientation = (hand_orientation * left_hand_xr_correction).into();

        self.pose_node_model(self.skeleton.left_arm.hand, hand_pose);

        self.cats_meow(self.skeleton.left_finger.index.proximal, left_hand.fingers[1][1], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.index.intermediate, left_hand.fingers[1][2], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.index.distal, left_hand.fingers[1][3], left_hand_xr_correction);

        self.cats_meow(self.skeleton.left_finger.middle.proximal, left_hand.fingers[2][1], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.middle.intermediate, left_hand.fingers[2][2], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.middle.distal, left_hand.fingers[2][3], left_hand_xr_correction);

        self.cats_meow(self.skeleton.left_finger.ring.proximal, left_hand.fingers[3][1], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.ring.intermediate, left_hand.fingers[3][2], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.ring.distal, left_hand.fingers[3][3], left_hand_xr_correction);

        self.cats_meow(self.skeleton.left_finger.little.proximal, left_hand.fingers[4][1], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.little.intermediate, left_hand.fingers[4][2], left_hand_xr_correction);
        self.cats_meow(self.skeleton.left_finger.little.distal, left_hand.fingers[4][3], left_hand_xr_correction);



        self.cats_meow(self.skeleton.left_finger.thumb.proximal, left_hand.fingers[0][1], left_hand_xr_correction_thumb);
        self.cats_meow(self.skeleton.left_finger.thumb.intermediate, left_hand.fingers[0][2], left_hand_xr_correction_thumb);
        self.cats_meow(self.skeleton.left_finger.thumb.distal, left_hand.fingers[0][3], left_hand_xr_correction_thumb);


        let hand_size = Mat4::from_quat(Quat::from(left_hand.wrist.orientation)).transform_point3(Vec3::new(0.0, 0.0, -left_hand.size * 2.0));

        let lower_arm_pose = Pose::new(Vec3::from(hand_pose.position).sub(hand_size).lerp(Vec3::from(sk.input_head().position), 0.25), Quat::from(hand_pose.orientation).lerp(Quat::from(self.ik.lower_left_arm.orientation), 0.25));

        self.mrow(self.skeleton.left_arm.lower_arm, lower_arm_pose, quat_from_angles(180.0, 0.0, 0.0));
        //self.pose_node_model(self.skeleton.left_arm.lower_arm, lower_arm_pose);
    }

    pub fn mrow(&mut self, node: NodeId, pose: Pose, correction_quat: Quat) {
        self.pose_node_model(node, Pose::new(pose.position, Quat::from(pose.orientation) * correction_quat));
    }

    pub fn cats_meow(&mut self, node: NodeId, joint: Joint, correction_quat: Quat) {
        // Refactor this!
        
        let mut tmp: Quat = joint.orientation.into();
    
        self.pose_node_model(node, Pose::new(joint.position, tmp*correction_quat));
    }

    pub fn do_other_ik(&mut self, sk: &StereoKitDraw) {
        let head_offset = &self.gltf.extensions.vrm.as_ref().unwrap().first_person.first_person_bone_offset;
        let head_offset = Vec3::new(head_offset.x, head_offset.y, head_offset.z);
        //println!("{}", head_offset);
        let mut sk_head = sk.input_head();
        sk_head.position = (Vec3::from(sk_head.position) ).into();
        let head = self.skeleton.head.head;
        self.model.node_set_transform_model(head, Mat4::from_scale_rotation_translation(Vec3::default(), sk_head.orientation.into(), sk_head.position.into()).into());
        let difference = self.ik.head - self.ik.hip;
        let (x, y, z) = angles_from_quat(sk_head.orientation.into());
        //let difference = difference + Vec3::new(0.0, 0.0, x / 180.0);
        self.pose_node_model(self.skeleton.torso.hips, Pose::new(Vec3::from(sk_head.position) - difference,  quat_from_angles(0.0, y, 0.0)));
        self.pose_node_model(self.skeleton.torso.chest, Pose::new(self.node_get_pose_model(self.skeleton.torso.chest).position, quat_from_angles(0.0, 0.0, 0.0)))
    }


    pub fn recursive_get_nodes_and_poses(&self, mut node: NodeId) -> Vec<(NodeId, Pose)> {
        let mut vec = vec![];
        vec.push((node, self.node_get_pose_model(node)));
        match self.model.node_child(node) {
            None => {}
            Some(child) => {
                vec.append(&mut self.recursive_get_nodes_and_poses(child));
            }
        }
        match self.model.node_sibling(node) {
            None => {}
            Some(sibling) => {
                vec.append(&mut self.recursive_get_nodes_and_poses(sibling));
            }
        }
        vec
    }
    pub fn set_nodes_and_poses(&mut self, nodes_and_poses: Vec<(NodeId, Pose)>) {
        for i in nodes_and_poses {
            self.pose_node_model(i.0, i.1);
        }
    }
    pub fn node_get_pose_model(&self, node_id: NodeId) -> Pose {
        let (s, q, t) = Mat4::from(self.model.node_get_transform_model(node_id)).to_scale_rotation_translation();
        Pose::new(t, q)
    }
    pub fn get_nodes_and_poses(&self, sk: &StereoKitDraw) -> Vec<(NodeId, Pose)> {
        let mut vector = vec![
            (self.skeleton.torso.hips, self.node_get_pose_model(self.skeleton.torso.hips)),
                (self.skeleton.head.head, sk.input_head())];
        vector.append(&mut self.recursive_get_nodes_and_poses(self.skeleton.right_arm.lower_arm));
        vector.append(&mut self.recursive_get_nodes_and_poses(self.skeleton.left_arm.lower_arm));
        vector
    }
    pub fn hide_nodes(&self, mut node: NodeId) {
        //println!("hiding node: {:?}", node);
        self.model.node_set_visible(node, false);
        match self.model.node_child(node) {
            None => {}
            Some(child) => {
                self.hide_nodes(child);
            }
        }
        match self.model.node_sibling(node) {
            None => {
                return;
            }
            Some(sibling) => {
                self.hide_nodes(sibling)
            }
        }
    }
}



pub fn joint_to_pose(joint: Joint) -> Pose {
    Pose::new(joint.position, joint.orientation)
}
pub fn apply_correction(mut pose: Pose, rotation: Quat) -> Pose {
    pose.orientation = rotation.mul_quat(pose.orientation.into()).into();
    pose
}
pub struct Ik {
    avatar_hand_size: Vec3,
    head_root_offset: Vec3,
    head: Vec3,
    hip: Vec3,
    lower_left_arm: Pose,
    lower_right_arm: Pose,
}
impl Ik {
    pub fn new(gltf: &VrmGltf, model: &Model, skeleton: &Skeleton) -> Self {


        let head_root_offset = &gltf.extensions.vrm.as_ref().unwrap().first_person.first_person_bone_offset;
        let head_root_offset = Vec3::new(head_root_offset.x, head_root_offset.y, head_root_offset.z);
        let head = Mat4::from(model.node_get_transform_model(skeleton.head.head)).to_scale_rotation_translation().2;
        let hip = Mat4::from(model.node_get_transform_model(skeleton.torso.hips)).to_scale_rotation_translation().2;
        let lower_left_arm = Mat4::from(model.node_get_transform_model(skeleton.left_arm.lower_arm)).to_scale_rotation_translation();
        let lower_left_arm = Pose::new(lower_left_arm.2, lower_left_arm.1);
        let lower_right_arm = Mat4::from(model.node_get_transform_model(skeleton.right_arm.lower_arm)).to_scale_rotation_translation();
        let lower_right_arm = Pose::new(lower_right_arm.2, lower_right_arm.1);
        Self {
            avatar_hand_size: Default::default(),
            head_root_offset,
            head,
            hip,
            lower_left_arm,
            lower_right_arm,
        }
    }
}

pub fn quat_from_angles(x: f32, y: f32, z: f32) -> Quat {
    Quat::from_euler(
        XYZ,
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