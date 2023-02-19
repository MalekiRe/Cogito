use std::ops::{Div, Neg};
use color_eyre::owo_colors::OwoColorize;
use glam::{EulerRot, Mat4, Quat, Vec3};
use glam::EulerRot::XYZ;
use goth_gltf::extensions::CompressionFilter::Quaternion;
use goth_gltf::Gltf;
use stereokit::color_named::{BLUE, GREEN, ORANGE, RED, YELLOW};
use stereokit::input::{Handed, Joint, StereoKitInput};
use stereokit::lifecycle::StereoKitDraw;
use stereokit::material::DEFAULT_ID_MATERIAL;
use stereokit::model;
use stereokit::model::{Model, NodeId};
use stereokit::pose::Pose;
use stereokit::render::{BackendXrType, RenderLayer, StereoKitRender};
use stereokit::ui::{ConfirmMethod, MoveType, window, WindowType};
use crate::{VrmAvatar, VrmGltf};
use crate::bones::{Skeleton};

impl VrmAvatar {
    pub fn update_ik(&mut self, sk: &StereoKitDraw) {

        //sk.input_hand_visible(Handed::Right, false);

        let right_hand = sk.input_hand(Handed::Right);

        let hand_scale = 1.0;

        let quat_correct_thumb = quat_from_angles(90.0, 90.0, 270.0); //90.0 90.0 270.0
        let quat_correct_node = quat_from_angles(90.0, 0.0, 180.0); //90.0 0.0 180.0
        let quat_flip = quat_from_angles(0.0, -90.0, 0.0);

        let r = angles_from_quat(sk.input_head().orientation.into());
        let q1 = quat_from_angles((self.r1 as u32 %27) as f32 * 90.0, (self.r1 as u32 %9)  as f32 * 90.0, (self.r1 as u32 %3)  as f32 * 90.0);
        let wrist_normalization = {
            match sk.backend_xr_get_type() {
                BackendXrType::Simulator => quat_flip,
                _ => q1,
            }
        };
        let mut hand_pose = right_hand.wrist;
        hand_pose = apply_correction(hand_pose, wrist_normalization);


        if self.n2 > 50 {
            self.n2 = 0;
            self.number += 1;
        }
        self.n2 += 1;


        self.r1 = 0.0;
       // self.r1 = self.number as f32; // 5.0 11.0 23.0

        let mat = Mat4::from(self.model.node_get_transform_local(self.skeleton.right_arm.hand));
        let (s, q, t) = mat.to_scale_rotation_translation();
        let r0 = Quat::from(right_hand.wrist.orientation);
        let r1 = Quat::from(right_hand.fingers[0][0].orientation);
        let r2 = Quat::from( right_hand.fingers[0][2].orientation);
        let r3 = Quat::from(right_hand.fingers[0][4].orientation);

        let t0 = Vec3::from(right_hand.fingers[0][0].position);
        let t2 = Vec3::from(right_hand.fingers[0][2].position);
        let t3 = Vec3::from(right_hand.fingers[0][3].position);
        let t4 = Vec3::from(right_hand.fingers[0][4].position);

        self.pose_node_local(self.skeleton.right_arm.hand, Pose::new(t, hand_pose.orientation));

        let mat = stereokit::material::Material::copy_from_id(sk, DEFAULT_ID_MATERIAL).unwrap();
        let mesh = stereokit::mesh::Mesh::gen_cube(sk, [0.01, 0.01, 0.01], 1).unwrap();
        let cube = model::Model::from_mesh(sk, &mesh, &mat).unwrap();

        cube.draw(sk, right_hand.wrist.as_matrix(),  RED, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[0][0]).as_matrix(),  ORANGE, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[0][1]).as_matrix(),  ORANGE, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[0][2]).as_matrix(),  GREEN, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[0][3]).as_matrix(),  GREEN, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[0][4]).as_matrix(),  BLUE, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[1][0]).as_matrix(),  ORANGE, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[1][1]).as_matrix(),  ORANGE, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[1][2]).as_matrix(),  GREEN, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[1][3]).as_matrix(),  GREEN, RenderLayer::Layer0);
        cube.draw(sk, joint_to_pose(right_hand.fingers[1][4]).as_matrix(),  BLUE, RenderLayer::Layer0);

        //self.pose_node_local(self.skeleton.right_finger.thumb.proximal, Pose::new(self.l_n_pos(self.skeleton.right_finger.thumb.proximal), r1.mul_quat(r0.inverse()).mul_quat(q1)));
        self.pose_node_local(self.skeleton.right_finger.thumb.proximal, Pose::new(t2 - t0, Quat::from(right_hand.fingers[0][2].orientation).mul_quat(Quat::from(right_hand.fingers[0][0].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.thumb.intermediate, Pose::new(t3 - t2, Quat::from(right_hand.fingers[0][3].orientation).mul_quat(Quat::from(right_hand.fingers[0][2].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.thumb.distal, Pose::new(t4 - t3, Quat::from(right_hand.fingers[0][4].orientation).mul_quat(Quat::from(right_hand.fingers[0][3].orientation).inverse())));

        self.pose_node_local(self.skeleton.right_finger.index.proximal, Pose::new(self.l_n_pos(self.skeleton.right_finger.index.proximal), Quat::from(right_hand.fingers[1][2].orientation).mul_quat(Quat::from(right_hand.fingers[1][1].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.index.intermediate, Pose::new(self.l_n_pos(self.skeleton.right_finger.index.intermediate), Quat::from(right_hand.fingers[1][3].orientation).mul_quat(Quat::from(right_hand.fingers[1][2].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.index.distal, Pose::new(self.l_n_pos(self.skeleton.right_finger.index.distal), Quat::from(right_hand.fingers[1][4].orientation).mul_quat(Quat::from(right_hand.fingers[1][2].orientation).inverse())));


        self.pose_node_local(self.skeleton.right_finger.middle.proximal, Pose::new(self.l_n_pos(self.skeleton.right_finger.middle.proximal), Quat::from(right_hand.fingers[2][2].orientation).mul_quat(Quat::from(right_hand.fingers[2][1].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.middle.intermediate, Pose::new(self.l_n_pos(self.skeleton.right_finger.middle.intermediate), Quat::from(right_hand.fingers[2][3].orientation).mul_quat(Quat::from(right_hand.fingers[2][2].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.middle.distal, Pose::new(self.l_n_pos(self.skeleton.right_finger.middle.distal), Quat::from(right_hand.fingers[2][4].orientation).mul_quat(Quat::from(right_hand.fingers[2][2].orientation).inverse())));

        self.pose_node_local(self.skeleton.right_finger.ring.proximal, Pose::new(self.l_n_pos(self.skeleton.right_finger.ring.proximal), Quat::from(right_hand.fingers[3][2].orientation).mul_quat(Quat::from(right_hand.fingers[3][1].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.ring.intermediate, Pose::new(self.l_n_pos(self.skeleton.right_finger.ring.intermediate), Quat::from(right_hand.fingers[3][3].orientation).mul_quat(Quat::from(right_hand.fingers[3][2].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.ring.distal, Pose::new(self.l_n_pos(self.skeleton.right_finger.ring.distal), Quat::from(right_hand.fingers[3][4].orientation).mul_quat(Quat::from(right_hand.fingers[3][2].orientation).inverse())));

        self.pose_node_local(self.skeleton.right_finger.little.proximal, Pose::new(self.l_n_pos(self.skeleton.right_finger.little.proximal), Quat::from(right_hand.fingers[4][2].orientation).mul_quat(Quat::from(right_hand.fingers[4][1].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.little.intermediate, Pose::new(self.l_n_pos(self.skeleton.right_finger.little.intermediate), Quat::from(right_hand.fingers[4][3].orientation).mul_quat(Quat::from(right_hand.fingers[4][2].orientation).inverse())));
        self.pose_node_local(self.skeleton.right_finger.little.distal, Pose::new(self.l_n_pos(self.skeleton.right_finger.little.distal), Quat::from(right_hand.fingers[4][4].orientation).mul_quat(Quat::from(right_hand.fingers[4][2].orientation).inverse())));
        //IMPORTANT self.pose_node_local(self.skeleton.right_finger.thumb.intermediate, Pose::new(self.l_n_pos(self.skeleton.right_finger.thumb.intermediate), r2.mul_quat(r1.inverse()).mul_quat(q1)));
        //IMPORTANT self.pose_node_local(self.skeleton.right_finger.thumb.distal, Pose::new(self.l_n_pos(self.skeleton.right_finger.thumb.distal), r3.mul_quat(r2.inverse()).mul_quat(q1)));

        //self.pose_node_local(self.skeleton.right_finger.index.proximal, Pose::new(self.l_n_pos(self.skeleton.right_finger.index.proximal), Quat::from(hand_pose.orientation).mul_quat(Quat::from(right_hand.fingers[1][0].orientation).inverse())));
        //self.pose_node_local(self.skeleton.right_finger.index.intermediate, Pose::new(self.l_n_pos(self.skeleton.right_finger.index.intermediate), Quat::from(right_hand.fingers[1][1].orientation).mul_quat(Quat::from(right_hand.fingers[1][2].orientation).inverse())));
        //self.pose_node_local(self.skeleton.right_finger.index.distal, Pose::new(self.l_n_pos(self.skeleton.right_finger.index.distal), Quat::from(right_hand.fingers[1][2].orientation).mul_quat(Quat::from(right_hand.fingers[1][4].orientation).inverse())));
        // let (_, _, f1) = Mat4::from(self.model.node_get_transform_local(self.skeleton.right_finger.index.distal)).to_scale_rotation_translation();
        // self.pose_node_local(self.skeleton.right_finger.index.distal, Pose::new(f1, right_hand.fingers[1][4].orientation));
        window::window(sk, "a", &mut Pose::IDENTITY, [0.5, 0.5].into(), WindowType::WindowNormal, MoveType::MoveNone, |ui| {
            ui.slider("self.r1", &mut self.r1, 0.0, 27.0, 1.0, 0.4, ConfirmMethod::Push);
            //ui.slider("self.r2", &mut self.r2, 0.0, 27.0, 1.0, 0.4, ConfirmMethod::Push);
            //ui.slider("x", &mut self.x, 0.0, 360.0, 90.0, 0.1, ConfirmMethod::Push);
            //ui.slider("y", &mut self.y, 0.0, 360.0, 90.0, 0.1, ConfirmMethod::Push);
            //ui.slider("z", &mut self.z, 0.0, 360.0, 90.0, 0.1, ConfirmMethod::Push);
        });
        // self.x = 90.0;
        // self.y = 180.0;
        // self.z = 270.0;
        // hand_pose = apply_correction(hand_pose, wrist_normalization);
        // self.r1 = self.number as f32;
        //
        // self.pose_node_local(self.skeleton.right_arm.hand, Pose::new(hand_pose.position, hand_pose.orientation));
        //
        // let r1 = Quat::from(right_hand.fingers[0][2].orientation);
        // let r2 = Quat::from(right_hand.fingers[0][4].orientation);
        // let p1 = Vec3::from(right_hand.fingers[0][2].position);
        // let p2 = Vec3::from(right_hand.fingers[0][4].position);
        // self.pose_node_local(self.skeleton.right_finger.thumb.distal, Pose::new(p2 - p1, r1.mul_quat(r2.inverse())));
        // //self.pose_node_model(self.skeleton.right_arm.hand, hand_pose);
        // let (mut x1, mut y1, mut z1) = angles_from_quat(right_hand.fingers[0][4].orientation.into());
        // let (x2, y2, z2) = angles_from_quat(right_hand.palm.orientation.into());
        // let r1 = Quat::from(right_hand.wrist.orientation);
        // let r2 = Quat::from(right_hand.fingers[0][0].orientation);
        // let p1 = Vec3::from(right_hand.wrist.position);
        // let p2 = Vec3::from(right_hand.fingers[0][0].position);
        // self.pose_node_local(self.skeleton.right_finger.thumb.proximal, Pose::new(p2-p1, r1.mul_quat(r2.inverse())));
        // let r1 = Quat::from(right_hand.fingers[0][0].orientation);
        // let r2 = Quat::from(right_hand.fingers[0][2].orientation);
        // let p1 = Vec3::from(right_hand.fingers[0][0].position);
        // let p2 = Vec3::from(right_hand.fingers[0][2].position);
        // self.pose_node_local(self.skeleton.right_finger.thumb.intermediate, Pose::new(p2 - p1, r1.mul_quat(r2.inverse())));

        // let p1 = Vec3::from(right_hand.fingers[0][0].position);
        // let p2 = Vec3::from(right_hand.fingers[0][2].position);
        // // x1 += x2;
        // // y1 += y2;
        // // z1 += z2;
        // self.pose_node_local(self.skeleton.right_finger.thumb.proximal, Pose::new(p2 - p1, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.thumb.intermediate, Pose::new(right_hand.fingers[0][2].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.thumb.distal, Pose::new(right_hand.fingers[0][4].position, quat_from_angles(x1, y1, z1)));

        //
        // self.pose_node_model(self.skeleton.right_finger.index.proximal, Pose::new(right_hand.fingers[1][0].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.index.intermediate, Pose::new(right_hand.fingers[1][2].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.index.distal, Pose::new(right_hand.fingers[1][4].position, quat_from_angles(x1, y1, z1)));
        //
        // self.pose_node_model(self.skeleton.right_finger.middle.proximal, Pose::new(right_hand.fingers[2][0].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.middle.intermediate, Pose::new(right_hand.fingers[2][2].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.middle.distal, Pose::new(right_hand.fingers[2][4].position, quat_from_angles(x1, y1, z1)));
        //
        // self.pose_node_model(self.skeleton.right_finger.ring.proximal, Pose::new(right_hand.fingers[3][0].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.ring.intermediate, Pose::new(right_hand.fingers[3][2].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.ring.distal, Pose::new(right_hand.fingers[3][4].position, quat_from_angles(x1, y1, z1)));
        //
        // self.pose_node_model(self.skeleton.right_finger.little.proximal, Pose::new(right_hand.fingers[4][0].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.little.intermediate, Pose::new(right_hand.fingers[4][2].position, quat_from_angles(x1, y1, z1)));
        // self.pose_node_model(self.skeleton.right_finger.little.distal, Pose::new(right_hand.fingers[4][4].position, quat_from_angles(x1, y1, z1)));
        //self.pose_node_model(self.skeleton.right_finger.thumb.intermediate, apply_correction(joint_to_pose(right_hand.fingers[0][2]), q2));
        //self.pose_node_model(self.skeleton.right_finger.thumb.distal, apply_correction(joint_to_pose(right_hand.fingers[0][4]), q2));
        //
        // self.pose_node_model(self.skeleton.right_finger.index.proximal, apply_correction(joint_to_pose(right_hand.fingers[1][0]), q2));
        // self.pose_node_model(self.skeleton.right_finger.index.intermediate, apply_correction(joint_to_pose(right_hand.fingers[1][2]), q2));
        // self.pose_node_model(self.skeleton.right_finger.index.distal, apply_correction(joint_to_pose(right_hand.fingers[1][4]), q2));
        //
        // self.pose_node_model(self.skeleton.right_finger.middle.proximal, apply_correction(joint_to_pose(right_hand.fingers[2][0]), q2));
        // self.pose_node_model(self.skeleton.right_finger.middle.intermediate, apply_correction(joint_to_pose(right_hand.fingers[2][2]), q2));
        // self.pose_node_model(self.skeleton.right_finger.middle.distal, apply_correction(joint_to_pose(right_hand.fingers[2][4]), q2));
        //
        // self.pose_node_model(self.skeleton.right_finger.ring.proximal, apply_correction(joint_to_pose(right_hand.fingers[3][0]), q2));
        // self.pose_node_model(self.skeleton.right_finger.ring.intermediate, apply_correction(joint_to_pose(right_hand.fingers[3][2]), q2));
        // self.pose_node_model(self.skeleton.right_finger.ring.distal, apply_correction(joint_to_pose(right_hand.fingers[3][4]), q2));
        //
        // self.pose_node_model(self.skeleton.right_finger.little.proximal, apply_correction(joint_to_pose(right_hand.fingers[4][0]), q2));
        // self.pose_node_model(self.skeleton.right_finger.little.intermediate, apply_correction(joint_to_pose(right_hand.fingers[4][2]), q2));
        // self.pose_node_model(self.skeleton.right_finger.little.distal, apply_correction(joint_to_pose(right_hand.fingers[4][4]), q2));

        //hand_pose = apply_correction(hand_pose, Quat::from(sk.input_head().orientation).inverse());

        // let wrist_mat = Mat4::from_scale_rotation_translation(
        //     Vec3::new(hand_scale, hand_scale, hand_scale),
        // Quat::from(hand_pose.orientation.clone()).mul_quat(
        //     wrist_normalization
        // ), hand_pose.position.into());


        //println!("{:#?}", right_hand);
        //
        // let mut p0 = hand_pose;
        // let mut p1 = joint_to_pose(right_hand.fingers[0][0]);
        // let mut p2 = joint_to_pose(right_hand.fingers[0][2]);
        // let mut p3 = joint_to_pose(right_hand.fingers[0][4]);
        //
        // p1 = apply_correction(apply_correction(p1, p0.orientation.into()), quat_correct_thumb);
        // p2 = apply_correction( apply_correction(p2, p0.orientation.into()), quat_correct_thumb);
        // p3 = apply_correction(apply_correction(p3, p0.orientation.into()), quat_correct_thumb);
        //
        //
        //
        //
        // self.pose_node_model(self.skeleton.right_arm.hand, p0);
        // self.pose_node_model(self.skeleton.right_finger.thumb.proximal, p1);
        // self.pose_node_model(self.skeleton.right_finger.thumb.intermediate, p2);
        // self.pose_node_model(self.skeleton.right_finger.thumb.distal, p3);
        //let a = a.mul_mat4(&Mat4::from_quat(sk.input_head().orientation.into()).inverse());
        // let a = Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), p1.orientation.into(), a.to_scale_rotation_translation().2);
        // self.transform_node_local(self.skeleton.right_finger.thumb.proximal, a);
        // //let b = b.mul_mat4(&Mat4::from_quat(sk.input_head().orientation.into()).inverse());
        // self.transform_node_local(self.skeleton.right_finger.thumb.intermediate, b);
        // //let c = c.mul_mat4(&Mat4::from_quat(sk.input_head().orientation.into()).inverse());
        // self.transform_node_local(self.skeleton.right_finger.thumb.distal, c);
        // let mut matrix: Mat4 = self.model.node_get_transform_local(self.skeleton.right_finger.thumb.intermediate).into();
        // let mut m2: Mat4 = self.model.node_get_transform_model(self.skeleton.right_finger.thumb.proximal).into();


        //self.transform_node_model(self.skeleton.right_arm.hand, wrist_mat);

        //self.pose_node_model(self.skeleton.right_finger.thumb.distal, apply_correction(apply_correction(joint_to_pose(right_hand.fingers[0][4]), quat_from_angles(r.0, r.1, r.2)), quat_correct_thumb));
        //self.pose_node_model(self.skeleton.right_finger.thumb.intermediate, apply_correction(apply_correction(joint_to_pose(right_hand.fingers[0][2]), quat_from_angles(r.0, r.1, r.2)), quat_correct_thumb));
        //self.pose_node_model(self.skeleton.right_finger.thumb.proximal, apply_correction(apply_correction(joint_to_pose(right_hand.fingers[0][0]), quat_from_angles(r.0, r.1, r.2)), quat_correct_thumb));

        let correction = quat_correct_node.mul_quat(sk.input_head().orientation.into());


        //self.pose_node_model(self.skeleton.right_finger.thumb.distal, apply_correction(joint_to_pose(right_hand.fingers[0][4]), quat_correct_thumb));
        //self.pose_node_model(self.skeleton.right_finger.thumb.intermediate, apply_correction(joint_to_pose(right_hand.fingers[0][2]), quat_correct_thumb));
        //self.pose_node_model(self.skeleton.right_finger.thumb.proximal, apply_correction(joint_to_pose(right_hand.fingers[0][0]), quat_correct_thumb));
        let a = quat_from_angles(r.0, r.1, r.2);
        //self.pose_node_model(self.skeleton.right_finger.index.distal, apply_correction(apply_correction(joint_to_pose(right_hand.fingers[1][4]), a), quat_correct_node));
        //self.pose_node_model(self.skeleton.right_finger.index.intermediate, apply_correction(apply_correction(joint_to_pose(right_hand.fingers[1][2]), a), quat_correct_node));
        //self.pose_node_model(self.skeleton.right_finger.index.proximal, apply_correction(apply_correction(joint_to_pose(right_hand.fingers[1][1]), a),quat_correct_node));
        //
        // self.pose_node_model(self.skeleton.right_finger.middle.distal, apply_correction(joint_to_pose(right_hand.fingers[2][4]), quat_correct_node));
        // self.pose_node_model(self.skeleton.right_finger.middle.intermediate, apply_correction(joint_to_pose(right_hand.fingers[2][2]), quat_correct_node));
        // self.pose_node_model(self.skeleton.right_finger.middle.proximal, apply_correction(joint_to_pose(right_hand.fingers[2][1]), quat_correct_node));
        //
        // self.pose_node_model(self.skeleton.right_finger.ring.distal, apply_correction(joint_to_pose(right_hand.fingers[3][4]), quat_correct_node));
        // self.pose_node_model(self.skeleton.right_finger.ring.intermediate, apply_correction(joint_to_pose(right_hand.fingers[3][2]), quat_correct_node));
        // self.pose_node_model(self.skeleton.right_finger.ring.proximal, apply_correction(joint_to_pose(right_hand.fingers[3][1]), quat_correct_node));
        //
        // self.pose_node_model(self.skeleton.right_finger.little.distal, apply_correction(joint_to_pose(right_hand.fingers[4][4]), quat_correct_node));
        // self.pose_node_model(self.skeleton.right_finger.little.intermediate, apply_correction(joint_to_pose(right_hand.fingers[4][2]), quat_correct_node));
        // self.pose_node_model(self.skeleton.right_finger.little.proximal, apply_correction(joint_to_pose(right_hand.fingers[4][1]), quat_correct_node));
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
}
impl Ik {
    pub fn new(gltf: &VrmGltf, model: &Model, skeleton: &Skeleton) -> Self {


        let head_root_offset = &gltf.extensions.vrm.as_ref().unwrap().first_person.first_person_bone_offset;
        let head_root_offset = Vec3::new(head_root_offset.x, head_root_offset.y, head_root_offset.z);

        Self {
            avatar_hand_size: Default::default(),
            head_root_offset,
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