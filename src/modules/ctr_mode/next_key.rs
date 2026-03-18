use crate::modules::aes_stuff::cipher::cipher;

pub(super) fn next_key(init_counter: &[u8;16], j: &usize, expanded_password: &[[u8;16];15]) -> [u8;16] {
    let mut next_counter: [u8;16]= init_counter.clone();
    let mut count: usize = 0;
    for i in 0..8 {
        count += (init_counter[15 - i] as usize)*((256 as usize).pow(i as u32));
    }
    count = j.wrapping_add(count);
    for i in 0..8 {
        next_counter[15 - i] =(count % 256) as u8;
        count = count / 256;
    }
    cipher(&mut next_counter, &expanded_password);
    next_counter
}