use sha2::{Sha256, Digest};

pub fn passw_to_array(password: &str) -> [u8;32] {
    let pass_sha: &[u8]= &Sha256::digest(password.as_bytes());
    let mut array: [u8;32] = [0; 32];
    array.copy_from_slice(pass_sha);
    array
}