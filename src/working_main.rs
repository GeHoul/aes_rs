mod modules;
use std::io;

use crate::modules::{file_to_vec, vec_to_file, passw_to_array, init_counter_gen, insert_seed, extract_seed};
use crate::modules::ctr_encrypt;

fn main() {
    let mut temp = String::new();
    println!("choose operation (encrypt/decrypt):");
    io::stdin().read_line(&mut temp).unwrap();
    let operation: bool = if temp.trim() == "encrypt" {true} 
                        else if temp.trim() == "decrypt" {false} 
                        else {panic!("error");};

    println!("choose file (full path)");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).unwrap();

    let mut data: Vec<u8> = file_to_vec(file_path.trim());

    println!("enter password");
    let mut password_temp = String::new();
    io::stdin().read_line(&mut password_temp).unwrap();
    let password: [u8;32] = passw_to_array(password_temp.trim());
    let init_counter: [u8;16] = if operation {init_counter_gen()}
                                else {extract_seed(&mut data)};

    ctr_encrypt(&mut data, &password, init_counter);

    if operation {
        insert_seed(&mut data, &init_counter);
    }

    vec_to_file(file_path.trim(), data);
}