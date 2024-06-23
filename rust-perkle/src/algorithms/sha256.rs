use crate::hasher;
use sha2::{digest::FixedOutput, Digest, Sha256};

#[derive(Clone)]
pub struct Sha256Algorithm {}

impl hasher::Hasher for Sha256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(data);
        <[u8; 32]>::from(hasher.finalize_fixed())
    }
}
