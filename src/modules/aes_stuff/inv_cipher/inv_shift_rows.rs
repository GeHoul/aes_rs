pub(super) fn inv_shift_rows(state: &mut [u8; 16]) {
    let r: [u8;16] = state.clone();
    *state = [r[0], r[13], r[10], r[7], 
            r[4], r[1], r[14], r[11], 
            r[8], r[5], r[2], r[15], 
            r[12], r[9], r[6], r[3]];
}