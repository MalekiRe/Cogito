// #[derive(Debug, Copy, Clone)]
// pub enum Bone {
//     Head,
//     RightHand,
//     RightThumbMetacarpal,
//     RightThumbProximal,
//     RightThumbDistal,
// }
//
// impl Bone {
//     pub fn to_vrm_str<'a>(self) -> &'a str {
//         match self {
//             Bone::Head => "head",
//             Bone::RightHand => "rightHand",
//             // TODO idk why but the spec isn't respected for this thumb part
//             Bone::RightThumbMetacarpal => "rightThumbIntermediate",
//             Bone::RightThumbProximal => "rightThumbProximal",
//             Bone::RightThumbDistal => "rightThumbDistal",
//         }
//     }
// }

use simple_error::SimpleResult;
use stereokit::model::NodeId;
use crate::{VrmAvatar, VrmGltf};


#[derive(Copy, Clone, Debug)]
pub struct Torso {
    pub hips: NodeId,
    pub spine: NodeId,
    pub chest: NodeId,
    pub upper_chest: NodeId,
    pub neck: NodeId,
}

#[derive(Copy, Clone, Debug)]
pub struct Head {
    head: NodeId,
    left_eye: Option<NodeId>,
    right_eye: Option<NodeId>,
    jaw: Option<NodeId>,
}

#[derive(Copy, Clone, Debug)]
pub struct Leg {
    upper_leg: NodeId,
    lower_leg: NodeId,
    foot: NodeId,
    toes: Option<NodeId>,
}

#[derive(Copy, Clone, Debug)]
pub struct Finger {
    pub thumb: FingerBone,
    pub index: FingerBone,
    pub ring: FingerBone,
    pub middle: FingerBone,
    pub little: FingerBone,
}

#[derive(Copy, Clone, Debug)]
pub struct FingerBone {
    pub intermediate: NodeId,
    pub proximal: NodeId,
    pub distal: NodeId,
}

#[derive(Copy, Clone, Debug)]
pub struct Arm {
    pub hand: NodeId,
    pub shoulder: Option<NodeId>,
    pub upper_arm: NodeId,
    pub lower_arm: NodeId,
}

#[derive(Copy, Clone, Debug)]
pub struct Skeleton {
    pub head: Head,
    pub torso: Torso,
    pub right_finger: Finger,
    pub left_finger: Finger,
    pub right_arm: Arm,
    pub left_arm: Arm,
    pub left_leg: Leg,
    pub right_leg: Leg,
}

impl Skeleton {
    pub fn new(gltf: &VrmGltf) -> SimpleResult<Self> {
        Ok(
            Skeleton {
                head: Head {
                    head: VrmAvatar::_get_node(gltf, "head")?,
                    left_eye: VrmAvatar::_get_node(gltf, "leftEye").ok(),
                    right_eye: VrmAvatar::_get_node(gltf, "rightEye").ok(),
                    jaw: VrmAvatar::_get_node(gltf, "jaw").ok(),
                },
                torso: Torso {
                    hips: VrmAvatar::_get_node(gltf, "hips")?,
                    spine: VrmAvatar::_get_node(gltf, "spine")?,
                    chest: VrmAvatar::_get_node(gltf, "chest")?,
                    upper_chest: VrmAvatar::_get_node(gltf, "upperChest")?,
                    neck: VrmAvatar::_get_node(gltf, "neck")?,
                },
                right_finger: Finger {
                    thumb: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "rightThumbIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "rightThumbProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "rightThumbDistal")?,
                    },
                    index: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "rightIndexIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "rightIndexProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "rightIndexDistal")?,
                    },
                    ring: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "rightRingIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "rightRingProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "rightRingDistal")?,
                    },
                    middle: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "rightMiddleIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "rightMiddleProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "rightMiddleDistal")?,
                    },
                    little: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "rightLittleIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "rightLittleProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "rightLittleDistal")?,
                    },
                },
                left_finger: Finger {
                    thumb: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "leftThumbIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "leftThumbProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "leftThumbDistal")?,
                    },
                    index: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "leftIndexIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "leftIndexProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "leftIndexDistal")?,
                    },
                    ring: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "leftRingIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "leftRingProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "leftRingDistal")?,
                    },
                    middle: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "leftMiddleIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "leftMiddleProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "leftMiddleDistal")?,
                    },
                    little: FingerBone {
                        intermediate: VrmAvatar::_get_node(gltf, "leftLittleIntermediate")?,
                        proximal: VrmAvatar::_get_node(gltf, "leftLittleProximal")?,
                        distal: VrmAvatar::_get_node(gltf, "leftLittleDistal")?,
                    },
                },
                right_arm: Arm {
                    hand: VrmAvatar::_get_node(gltf, "rightHand")?,
                    shoulder: VrmAvatar::_get_node(gltf, "rightShoulder").ok(),
                    upper_arm: VrmAvatar::_get_node(gltf, "rightUpperArm")?,
                    lower_arm: VrmAvatar::_get_node(gltf, "rightLowerArm")?,
                },
                left_arm: Arm {
                    hand: VrmAvatar::_get_node(gltf, "leftHand")?,
                    shoulder: VrmAvatar::_get_node(gltf, "leftShoulder").ok(),
                    upper_arm: VrmAvatar::_get_node(gltf, "leftUpperArm")?,
                    lower_arm: VrmAvatar::_get_node(gltf, "leftLowerArm")?,
                },
                left_leg: Leg {
                    upper_leg: VrmAvatar::_get_node(gltf, "leftUpperLeg")?,
                    lower_leg: VrmAvatar::_get_node(gltf, "leftLowerLeg")?,
                    foot: VrmAvatar::_get_node(gltf, "leftFoot")?,
                    toes: VrmAvatar::_get_node(gltf, "leftToes").ok(),
                },
                right_leg: Leg {
                    upper_leg: VrmAvatar::_get_node(gltf, "rightUpperLeg")?,
                    lower_leg: VrmAvatar::_get_node(gltf, "rightLowerLeg")?,
                    foot: VrmAvatar::_get_node(gltf, "rightFoot")?,
                    toes: VrmAvatar::_get_node(gltf, "rightToes").ok(),
                },
            }
        )
    }
}