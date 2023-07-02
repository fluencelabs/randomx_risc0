use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interface {
    pub key: String,
    pub nonce: u32
}

impl Interface {
    pub fn new(key: String, nonce: u32) -> Self {
        Self {
            key,
            nonce
        }
    }
}
