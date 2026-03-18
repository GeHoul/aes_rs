use super::byte_inverse::byte_inverse;

pub fn sbox(b: u8) -> u8 {
    let s: u8 = byte_inverse(&b);
    s ^ s.rotate_left(1) ^ s.rotate_left(2) ^ s.rotate_left(3) ^ s.rotate_left(4) ^ 0b01100011
}