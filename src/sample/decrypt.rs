mod modules;
use std::io;

use crate::modules::{file_to_vec, vec_to_file, passw_to_array, init_counter_gen, insert_seed, extract_seed};
use crate::modules::ctr_encrypt;


fn main() {
    println!("enter fake file path");
    let mut fake_path = String::new();
    io::stdin().read_line(&mut fake_path).unwrap();

    println!("enter fake copy path");
    let mut fake_path_copy = String::new();
    io::stdin().read_line(&mut fake_path_copy).unwrap();


    let mut fake_data: Vec<u8> = file_to_vec(fake_path.trim());

    let fake_data_copy: Vec<u8> = file_to_vec(fake_path_copy.trim());

    for i in 0..fake_data.len() {
        fake_data[i] = fake_data[i] ^ fake_data_copy[i];
    }

    println!("enter encrypted file path");
    let mut enc_path = String::new();
    io::stdin().read_line(&mut enc_path).unwrap();

    let mut enc_data: Vec<u8> = file_to_vec(enc_path.trim());

    for i in 0..enc_data.len() {
        enc_data[i] = fake_data[i] ^ enc_data[i];
    }
    
    vec_to_file(enc_path.trim(), enc_data);
}