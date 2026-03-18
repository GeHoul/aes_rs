use crate::modules::aes_stuff::cipher::cipher;
use crate::modules::aes_stuff::key::key_expansion;
use super::next_key::next_key;

pub fn ctr_encrypt(data: &mut Vec<u8>, password: &[u8;32], init_counter: [u8;16]) {
    
    let expanded_password: [[u8;16];15] = key_expansion(&password);
    let mut next_counter: [u8;16] = init_counter.clone();
    cipher(&mut next_counter, &expanded_password);

    let c: usize = data.len() / 16;
    let r: usize = data.len() % 16;
    let mut j: usize = 0;

    while j < c {
        for i in 0..16 {
            data[j*16+i] ^= next_counter[i];
        }
        j += 1;
        next_counter = next_key(&init_counter, &j, &expanded_password);       
    }
    if r != 0 {
        for i in 0..r {
            data[j*16+i] ^= next_counter[i];
        }
    }
}