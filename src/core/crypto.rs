use md5;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod arc_four_cipher;

pub use arc_four_cipher::ArcFourCipher;

pub fn calculate_md5(data: &[u8], offset: usize, length: usize) -> [u8; 16] {
    md5::compute(&data[offset..(offset + length)]).into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = calculateMD5))]
pub fn calculate_md5_wasm(data: &[u8], offset: usize, length: usize) -> Vec<u8> {
    calculate_md5(dbg!(data), offset, length).to_vec()
}
