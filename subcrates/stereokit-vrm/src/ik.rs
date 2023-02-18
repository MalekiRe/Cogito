use glam::Quat;
use goth_gltf::extensions::CompressionFilter::Quaternion;
use goth_gltf::Gltf;
use stereokit::input::{Handed, Joint, StereoKitInput};
use stereokit::lifecycle::StereoKitDraw;
use stereokit::model::NodeId;
use stereokit::pose::Pose;
use crate::{VrmAvatar, VrmGltf};
use crate::bones::{Skeleton};

impl VrmAvatar {
    pub fn update_ik(&mut self, sk: &StereoKitDraw) {

        sk.input_hand_visible(Handed::Right, false);

        let right_hand = sk.input_hand(Handed::Right);

        self.pose_node_model(self.skeleton.right_arm.hand, right_hand.wrist);

        self.pose_node_model(self.skeleton.right_finger.thumb.distal, joint_to_pose(right_hand.fingers[0][4]));
        self.pose_node_model(self.skeleton.right_finger.thumb.intermediate, joint_to_pose(right_hand.fingers[0][2]));
        self.pose_node_model(self.skeleton.right_finger.thumb.proximal, joint_to_pose(right_hand.fingers[0][0]));

        self.pose_node_model(self.skeleton.right_finger.index.distal, joint_to_pose(right_hand.fingers[1][4]));
        self.pose_node_model(self.skeleton.right_finger.index.intermediate, joint_to_pose(right_hand.fingers[1][2]));
        self.pose_node_model(self.skeleton.right_finger.index.proximal, joint_to_pose(right_hand.fingers[1][1]));

        self.pose_node_model(self.skeleton.right_finger.middle.distal, joint_to_pose(right_hand.fingers[2][4]));
        self.pose_node_model(self.skeleton.right_finger.middle.intermediate, joint_to_pose(right_hand.fingers[2][2]));
        self.pose_node_model(self.skeleton.right_finger.middle.proximal, joint_to_pose(right_hand.fingers[2][1]));

        self.pose_node_model(self.skeleton.right_finger.ring.distal, joint_to_pose(right_hand.fingers[3][4]));
        self.pose_node_model(self.skeleton.right_finger.ring.intermediate, joint_to_pose(right_hand.fingers[3][2]));
        self.pose_node_model(self.skeleton.right_finger.ring.proximal, joint_to_pose(right_hand.fingers[3][1]));

        self.pose_node_model(self.skeleton.right_finger.little.distal, joint_to_pose(right_hand.fingers[4][4]));
        self.pose_node_model(self.skeleton.right_finger.little.intermediate, joint_to_pose(right_hand.fingers[4][2]));
        self.pose_node_model(self.skeleton.right_finger.little.proximal, joint_to_pose(right_hand.fingers[4][1]));
    }
}

pub fn joint_to_pose(joint: Joint) -> Pose {
    Pose::new(joint.position, Quat::IDENTITY)
}
