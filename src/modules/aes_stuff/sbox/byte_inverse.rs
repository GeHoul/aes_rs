use super::gf16::{f_16mult, f_16inv};

pub fn byte_inverse(b: &u8) -> u8 {
    let mut m0: u8 = (0b00000001 & *b).wrapping_neg();
    let mut m1: u8 = ((0b00000010 & *b) >> 1).wrapping_neg();
    let mut m2: u8 = ((0b00000100 & *b) >> 2).wrapping_neg();
    let mut m3: u8 = ((0b00001000 & *b) >> 3).wrapping_neg();
    let mut m4: u8 = ((0b00010000 & *b) >> 4).wrapping_neg();
    let mut m5: u8 = ((0b00100000 & *b) >> 5).wrapping_neg();
    let mut m6: u8 = ((0b01000000 & *b) >> 6).wrapping_neg();
    let mut m7: u8 = ((0b10000000 & *b) >> 7).wrapping_neg();

    let mut s: u8 = (m0 & 0b00000001) ^ (m1 & 0b00101100) ^
    (m2 & 0b01001101) ^ (m3 & 0b01000111) ^
    (m4 & 0b00110110) ^ (m5 & 0b11011101) ^
    (m6 & 0b00111110) ^ (m7 & 0b11100111);

    let g: u8 = s & 0b11110000;
    let f: u8 = s & 0b00001111;
    let gg: u8 = f_16mult(&(g ^ (g >> 4)));
    let ff: u8 = f_16mult(&(f ^ (f << 4)));
    let inverse: u8 = f_16inv(&(f_16mult(&(gg ^ 0b10010000)) ^ f_16mult(&s) ^ ff));
    s = (f_16mult(&(g ^ inverse)) << 4) ^ f_16mult(&(((f << 4) ^ g) ^ inverse));

    m0 = (0b00000001 & s).wrapping_neg();
    m1 = ((0b00000010 & s) >> 1).wrapping_neg();
    m2 = ((0b00000100 & s) >> 2).wrapping_neg();
    m3 = ((0b00001000 & s) >> 3).wrapping_neg();
    m4 = ((0b00010000 & s) >> 4).wrapping_neg();
    m5 = ((0b00100000 & s) >> 5).wrapping_neg();
    m6 = ((0b01000000 & s) >> 6).wrapping_neg();
    m7 = ((0b10000000 & s) >> 7).wrapping_neg();

    s = (m0 & 0b00000001) ^ (m1 & 0b01011100) ^
    (m2 & 0b11100000) ^ (m3 & 0b01010000) ^
    (m4 & 0b00011110) ^ (m5 & 0b10110010) ^
    (m6 & 0b10110101) ^ (m7 & 0b00111010);
    s
}