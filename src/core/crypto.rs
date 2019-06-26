use md5;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod arc_four_cipher;

pub use arc_four_cipher::ArcFourCipher;

pub fn calculate_md5(data: &[u8], offset: usize, length: usize) -> [u8; 16] {
    md5::compute(&data[offset..(offset + length)]).into()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = calculateMD5)]
pub fn calculate_md5_wasm(data: Uint8Array, offset: usize, length: usize) -> Uint8Array {
    let mut data_vec = Vec::with_capacity(data.length() as usize);
    data.copy_to(&mut data_vec);
    let slice = calculate_md5(&data_vec, offset, length).to_vec();
    // TODO: Use from when it is available
    unsafe { Uint8Array::new(&Uint8Array::view(&slice)) }
}
