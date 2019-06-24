use wasm_bindgen::prelude::*;

const MAX_ADLER32_LENGTH: usize = 5552;

#[wasm_bindgen(js_name = computeAdler32)]
pub fn compute_adler_32(bytes: &[u8]) -> u32 {
    let bytes_length = bytes.len();
    debug_assert!(
        bytes_length < MAX_ADLER32_LENGTH,
        "computeAdler32: Unsupported bytes length."
    );
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for byte in bytes.iter().map(|c| u32::from(*c)) {
        a = a.wrapping_add(byte & 0xFF);
        b = b.wrapping_add(a);
    }
    ((b % 65521) << 16) | (a % 65521)
}
