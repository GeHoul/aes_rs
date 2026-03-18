mod byte_inverse;
mod gf16;
mod sbox;
mod inv_sbox;

pub(in crate::modules::aes_stuff) use sbox::sbox;
pub(in crate::modules::aes_stuff) use inv_sbox::inv_sbox;