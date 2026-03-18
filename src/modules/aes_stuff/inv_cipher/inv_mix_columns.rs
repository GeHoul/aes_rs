use crate::modules::aes_stuff::cipher::mix_columns;

pub(super) fn inv_mix_columns(state: &mut [u8; 16]) {
    mix_columns(state);
    mix_columns(state);
    mix_columns(state);
}