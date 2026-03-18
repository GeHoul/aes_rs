use super::{shift_rows::shift_rows, mix_columns::mix_columns};
use crate::modules::aes_stuff::sbox::sbox;
use crate::modules::aes_stuff::add_round_key::add_round_key;

pub(crate) fn cipher(state: &mut [u8;16], key: &[[u8;16];15]) {
    add_round_key(state, &key[0]);
    for i in 1..14 {
        *state = state.map(sbox); //sub_bytes
        shift_rows(state);
        mix_columns(state);
        add_round_key(state, &key[i]);
    }
    *state = state.map(sbox);  //sub_bytes
    shift_rows(state);
    add_round_key(state, &key[14]);
}