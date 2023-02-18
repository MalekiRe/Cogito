#[derive(Debug, Copy, Clone)]
pub enum Bone {
    Head,
    RightHand,
}

impl Bone {
    pub fn to_vrm_str<'a>(self) -> &'a str {
        match self {
            Bone::Head => "head",
            Bone::RightHand => "rightHand",
        }
    }
}