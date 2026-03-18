mod modules;
use std::io;

use crate::modules::{file_to_vec, vec_to_file, passw_to_array, init_counter_gen, insert_seed, extract_seed};
use crate::modules::ctr_encrypt;


fn main() {
    println!("enter fake file path");
    let mut fake_path = String::new();
    io::stdin().read_line(&mut fake_path).unwrap();

    println!("enter encrypted file path");
    let mut enc_path = String::new();
    io::stdin().read_line(&mut enc_path).unwrap();

    let mut enc_data: Vec<u8> = file_to_vec(enc_path.trim());

    let mut fake_data: Vec<u8> = file_to_vec(fake_path.trim());

    let init_counter: [u8;16] = extract_seed(&mut enc_data);

    insert_seed(&mut fake_data, &init_counter);

    vec_to_file(enc_path.trim(), enc_data);
    vec_to_file(fake_path.trim(), fake_data);
}