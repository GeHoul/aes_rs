use rand::RngCore;

pub fn init_counter_gen() -> [u8;16] {
    let mut array: [u8;16] = [0;16];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut array);
    array
}