pub fn f_16mult(b: &u8) -> u8 {
    let b1: u8 = (*b & 0b11110000) >> 4;
    let b2: u8 = *b & 0b00001111;
    let d3: u8 = 0u8.wrapping_sub((b1 >> 3) & 1);
    let d2: u8 = 0u8.wrapping_sub((b1 >> 2) & 1);
    let d1: u8 = 0u8.wrapping_sub((b1 >> 1) & 1);
    let mut d0: u8 = 0u8.wrapping_sub(b1 & 1);

    let mut s: u8 = ((b2 << 3) & d3) ^ ((b2 << 2) & d2) ^ ((b2 << 1) & d1) ^ (b2 & d0);

    d0 = s & 0b01110000;
    s ^= d0;
    s ^= d0 >> 3;
    s ^= d0 >> 4;
    s
}