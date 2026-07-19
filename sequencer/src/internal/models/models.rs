
#[derive(Clone, Debug)]
pub struct RootPayload {
    pub certificate_root: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}


