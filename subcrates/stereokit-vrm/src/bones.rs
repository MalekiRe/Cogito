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

use stereokit::model::NodeId;
use crate::{VrmAvatar, VrmGltf};



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
}

#[derive(Copy, Clone, Debug)]
pub struct Skeleton {
    pub right_finger: Finger,
    pub left_finger: Finger,
    pub right_arm: Arm,
    pub left_arm: Arm,
}

impl Skeleton {
    pub fn new(gltf: &VrmGltf) -> Option<Self> {
        Some(
            Skeleton {
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
                    hand: VrmAvatar::_get_node(gltf, "rightHand")?
                },
                left_arm: Arm {
                    hand: VrmAvatar::_get_node(gltf, "leftHand")?,
                },
            }
        )
    }
}