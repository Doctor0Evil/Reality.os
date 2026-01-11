use serde::Serialize;
use sha2::{Digest, Sha256};
use hex::encode as hex_encode;

pub fn hash_json_sha256<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_vec(value).expect("serialize");
    let mut hasher = Sha256::new();
    hasher.update(&json);
    hex_encode(hasher.finalize())
}
