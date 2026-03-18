use super::byte_inverse::byte_inverse;

pub fn inv_sbox(b: u8) -> u8 {
    let mut s: u8 = b ^ 0b01100011; 
    s = s.rotate_left(1) ^ s.rotate_left(3) ^ s.rotate_left(6);
    byte_inverse(&s)
}