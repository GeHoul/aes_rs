use crate::modules::aes_stuff::sbox::sbox;

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

pub fn key_expansion(key: &[u8;32]) -> [[u8;16];15] {
    let mut w: [u8;240] = [0;240];
    w[..32].copy_from_slice(key);
    let mut temp: [u8;4] = [0;4];
    let mut i=8;
    while i < 60 {
        temp[..4].copy_from_slice(&w[4*(i-1)..4*i]);
        if i % 8 == 0 {
            temp = [temp[1],temp[2],temp[3],temp[0]];    //RotWord
            temp = temp.map(sbox); //SubWord
            temp[0] = temp[0] ^ RCON[i/8 - 1];
        }
        else if i % 8 == 4 {
            temp = temp.map(sbox); //SubWord
        }
        for j in 0..4 {
            w[4*i+j]=w[4*(i-8)+j]^temp[j];
        }
        i += 1;
    }

    let mut round_keys: [[u8;16];15] = [[0;16];15];
    for i in 0..15 {
        for r in 0..4 {
            for c in 0..4 {
                round_keys[i][4*c + r] = w[16*i + 4*c +r];
            }
        }
    }
    round_keys
}