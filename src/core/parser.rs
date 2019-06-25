const MAX_ADLER32_LENGTH: usize = 5552;

pub fn compute_adler_32(bytes: &[u8]) -> u32 {
    debug_assert!(
        bytes.len() < MAX_ADLER32_LENGTH,
        "computeAdler32: Unsupported \"bytes\" length."
    );

    let (a, b) = bytes.iter().fold((1u16, 0u16), |(mut a, mut b), &byte| {
        a = a.wrapping_add(u16::from(byte));
        b = b.wrapping_add(a);
        (a, b)
    });

    u32::from(b) << 16 | u32::from(a)
}

#[test]
fn test_compute_adler_32() {
    assert_eq!(compute_adler_32("Wikipedia".as_bytes()), 0x11E60398);
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = computeAdler32)]
    pub fn compute_adler_32(bytes: &[u8]) -> u32 {
        super::compute_adler_32(bytes)
    }
}
