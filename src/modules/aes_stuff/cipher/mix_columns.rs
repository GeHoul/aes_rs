pub(in crate::modules::aes_stuff) fn mix_columns(state: &mut [u8; 16]) {
    let xt = |b: u8| (b << 1) ^ (((b >> 7) & 1).wrapping_neg() & 0x1b);
    for c in 0..4 {
        let s0 = state[4*c];
        let s1 = state[4*c+1];
        let s2 = state[4*c+2];
        let s3 = state[4*c+3];

        state[4*c] = xt(s0 ^ s1) ^ s1 ^ s2 ^ s3;
        state[4*c + 1] = xt(s1 ^ s2) ^ s2 ^ s3 ^ s0;
        state[4*c + 2] = xt(s2 ^ s3) ^ s3 ^ s0 ^ s1;
        state[4*c + 3] = xt(s3 ^ s0) ^ s0 ^ s1 ^ s2;
    }
}