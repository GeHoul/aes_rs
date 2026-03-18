mod aes_stuff;
mod ctr_mode;
mod data_preperation;

pub use ctr_mode::ctr_encrypt::ctr_encrypt;
pub use data_preperation::{file_management::{extract_seed, file_to_vec, vec_to_file, insert_seed}, 
                            init_counter_gen::init_counter_gen, passw_to_array::passw_to_array};