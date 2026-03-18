fn mult(b: &u8) -> u8 {
    let f1: u8 = *b & 0b00000011;
    let f2: u8 = (*b & 0b00001100) >> 2;
    let f3: u8 = f1 & f2;
    let f: u8 = ((f2 << 1) & f1) ^ ((f1 << 1) & f2) ^ (f3 & 0b00000010) ^ ((f3 >>1) ^ (f3 & 0b00000001));
    f
}

fn inv(b: &u8) -> u8 {
    let f:u8 = ((*b >> 1) & 0b00000001) ^ *b;
    f
}

pub fn f_16inv(b: &u8) -> u8 {
    let mut a0: u8 = *b & 0b00000001;
    let mut a1: u8 = *b & 0b00000010;
    let mut a2: u8 = *b & 0b00000100;
    let mut a3: u8 = *b & 0b00001000;
    let mut f: u8 = a3 ^ (a3 >> 1) ^ a2 ^ (a1 << 1) ^ (a2 >> 1) ^ (a3 >> 2) ^ a0;

    let b0: u8 = f & 0b00000011;
    let c: u8 = f & 0b00001100;
    let inverse: u8 = inv(&(mult(&(0b00001000 ^ inv(&(c >> 2)))) ^ mult(&f) ^ inv(&b0)));
    f = (mult(&(c ^ inverse)) << 2) ^ mult(&((c ^ (b0 << 2)) ^ inverse));

    a3 = f & 0b00001000;
    a2 = ((f & 0b00000010) ^ (a3 >> 2)) << 1;
    a1 = ((f & 0b00000100) ^ a2 ^ (a3 >> 1)) >> 1;
    a0 = f & 0b00000001;
    f = a0 ^ a1 ^ a2 ^ a3;
    f
}