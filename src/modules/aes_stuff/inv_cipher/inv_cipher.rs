use super::{inv_shift_rows::inv_shift_rows, inv_mix_columns::inv_mix_columns};
use crate::modules::aes_stuff::sbox::inv_sbox;
use crate::modules::aes_stuff::add_round_key::add_round_key;

pub(crate) fn inv_cipher(state: &mut [u8;16], key: &[[u8;16];15]){
    add_round_key(state, &key[0]);
    inv_shift_rows(state);
    *state = state.map(inv_sbox); //inv_sub_bytes
    for round in 1..14 {
        add_round_key(state, &key[round]);
        inv_mix_columns(state);
        inv_shift_rows(state);
        *state = state.map(inv_sbox); //inv_sub_bytes
    }
    add_round_key(state, &key[14]);
}