use super::key_expansion::key_expansion;

pub(crate) fn inv_key_expansion(key: &[u8;32]) -> [[u8;16];15] {
    let mut round_keys: [[u8;16];15] = key_expansion(&key);
    let r:[[u8;16];15] = round_keys.clone();
    for i in 0..15 {
        round_keys[14-i] = r[i]
    }
    round_keys
}