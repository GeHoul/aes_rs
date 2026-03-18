pub(super) fn add_round_key(state: &mut [u8;16], roundkey: &[u8;16]) {
    for i in 0..16 {
        state[i] ^= roundkey [i];
    }
}