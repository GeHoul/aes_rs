use std::fs;

pub fn file_to_vec(file_path: &str) -> Vec<u8> {
    fs::read(file_path).expect("failed to read the file")
}

pub fn vec_to_file(file_path: &str, data: Vec<u8>) {
    fs::write(file_path, data).expect("failed to write the file");
}

pub fn insert_seed(data: &mut Vec<u8>, init_counter: &[u8;16]) {
    for i in 0..16 {
        data.insert(0,init_counter[15-i]);
    }    
}

pub fn extract_seed(data: &mut Vec<u8>) -> [u8;16] {
    let mut init_counter: [u8;16] = [0;16];
    for i in 0..16 {
        init_counter[i] = data[0];
        data.remove(0);
    }
    init_counter
}