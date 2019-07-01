#[cfg(target_arch = "wasm32")]
use wasm_bindgen::cast::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use byteorder::{NativeEndian, ReadBytesExt};

const SEED: u32 = 0xc3d2e1f0;

const MASK_HIGH: u32 = 0xffff0000;
const MASK_LOW: u32 = 0xffff;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct MurmurHash3_64 {
    h1: u32,
    h2: u32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MurmurHash3_64 {
    pub fn new() -> Self {
        Self::new_from_seed(SEED)
    }

    pub fn new_from_seed(seed: u32) -> Self {
        MurmurHash3_64 { h1: seed, h2: seed }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new_with_maybe_seed(seed: Option<u32>) -> Self {
        Self::new_from_seed(seed.unwrap_or(SEED))
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = update)]
    pub fn update_wasm_bindgen(&mut self, input: JsValue) -> Result<(), JsValue> {
        if let Some(s) = input.as_string() {
            Ok(self.update(s))
        } else if let Ok(array_buffer) = input.dyn_into::<js_sys::ArrayBuffer>() {
            let bytes: Vec<u8> = (0..array_buffer.byte_length())
                .map(|index| js_sys::Reflect::get_u32(&array_buffer, index))
                .collect();

            Ok(self.update(bytes))
        } else {
            Err(js_sys::Error::new(
                "Wrong data format in MurmurHash3_64_update. Input must be a string or array.",
            )
            .into())
        }
    }

    pub fn update<In: AsRef<[u8]>>(&mut self, data_as_ref: In) {
        let data = data_as_ref.as_ref();
        let block_counts = data.len() / 4;
        let tail_length = data.len() - block_counts * 4;

        let mut data_u32: Vec<u32> = data
            .chunks_exact(4)
            .map(|mut slice| {
                slice
                    .read_u32::<NativeEndian>()
                    .expect("could not read u32 from array")
            })
            .collect();
        let mut k1;
        let mut k2;
        let mut h1 = self.h1;
        let mut h2 = self.h2;

        const C1: u32 = 0xcc9e2d51;
        const C2: u32 = 0x1b873593;
        const C1_LOW: u32 = C1 & MASK_LOW;
        const C2_LOW: u32 = C2 & MASK_LOW;

        for i in 0..block_counts {
            if i & 1 != 0 {
                k1 = data_u32[i];
                k1 = (k1.wrapping_mul(C1) & MASK_HIGH) | (k1.wrapping_mul(C1_LOW) & MASK_LOW);
                k1 = k1 << 15 | k1 >> 17;
                k1 = (k1.wrapping_mul(C2) & MASK_HIGH) | (k1.wrapping_mul(C2_LOW) & MASK_LOW);
                h1 ^= k1;
                h1 = h1 << 13 | h1 >> 19;
                h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
            } else {
                k2 = data_u32[i];
                k2 = (k2.wrapping_mul(C1) & MASK_HIGH) | (k2.wrapping_mul(C1_LOW) & MASK_LOW);
                k2 = k2 << 15 | k2 >> 17;
                k2 = (k2.wrapping_mul(C2) & MASK_HIGH) | (k2.wrapping_mul(C2_LOW) & MASK_LOW);
                h2 ^= k2;
                h2 = h2 << 13 | h2 >> 19;
                h2 = h2.wrapping_mul(5).wrapping_add(0xe6546b64);
            }
        }

        k1 = 0;

        if tail_length == 3 {
            k1 ^= u32::from(data[block_counts * 4 + 2]) << 16;
        }

        if tail_length == 3 || tail_length == 2 {
            k1 ^= u32::from(data[block_counts * 4 + 1]) << 8;
        }

        if tail_length == 3 || tail_length == 2 || tail_length == 1 {
            k1 ^= u32::from(data[block_counts * 4]);

            k1 = (k1.wrapping_mul(C1) & MASK_HIGH) | (k1.wrapping_mul(C1_LOW) & MASK_LOW);
            k1 = k1 << 15 | k1 >> 17;
            k1 = (k1.wrapping_mul(C2) & MASK_HIGH) | (k1.wrapping_mul(C2_LOW) & MASK_LOW);
            if block_counts & 1 != 0 {
                h1 ^= k1;
            } else {
                h2 ^= k1;
            }
        }

        self.h1 = h1;
        self.h2 = h2;
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn hexdigest(&mut self) -> String {
        let mut h1 = self.h1;
        let mut h2 = self.h2;

        h1 ^= h2 >> 1;
        h1 = (h1.wrapping_mul(0xed558ccd) & MASK_HIGH) | (h1.wrapping_mul(0x8ccd) & MASK_LOW);
        h2 = (h2.wrapping_mul(0xff51afd7) & MASK_HIGH)
            | (((h2 << 16 | h1 >> 16).wrapping_mul(0xafd7ed55) & MASK_HIGH) >> 16);
        h1 ^= h2 >> 1;
        h1 = (h1.wrapping_mul(0x1a85ec53) & MASK_HIGH) | (h1.wrapping_mul(0xec53) & MASK_LOW);
        h2 = (h2.wrapping_mul(0xc4ceb9fe) & MASK_HIGH)
            | (((h2 << 16 | h1 >> 16).wrapping_mul(0xb9fe1a85) & MASK_HIGH) >> 16);
        h1 ^= h2 >> 1;

        format!("{:x}{:x}", (h1 >> 0), (h2 >> 0))
    }
}

#[cfg(test)]
mod test {
    //! This module is based on `murmurhash3_spec.js`.

    use super::MurmurHash3_64;

    #[test]
    fn it_instantiates_without_seed() {
        let hash = MurmurHash3_64::new();
        let hash2 = MurmurHash3_64::new_with_maybe_seed(None);
    }

    #[test]
    fn it_instantiated_with_seed() {
        let hash = MurmurHash3_64::new_from_seed(1);
        let hash2 = MurmurHash3_64::new_with_maybe_seed(Some(1));
    }

    const hex_digest_expected: &str = "f61cfdbfdae0f65e";
    const source_text: &str = "test";
    const source_char_codes: [u8; 4] = [116, 101, 115, 116];

    #[test]
    fn correctly_generates_a_hash_from_a_string() {
        let mut hash = MurmurHash3_64::new();
        hash.update(source_text);
        assert_eq!(hash.hexdigest(), hex_digest_expected);
    }

    #[test]
    fn correctly_generates_a_hash_from_an_array() {
        let mut hash = MurmurHash3_64::new();
        hash.update(source_char_codes);
        assert_eq!(hash.hexdigest(), hex_digest_expected);
    }

    #[test]
    fn changes_the_hash_after_update_without_seed() {
        let mut hash = MurmurHash3_64::new();
        hash.update(source_text);
        let hex_digest_1 = hash.hexdigest();
        hash.update(source_text);
        let hex_digest_2 = hash.hexdigest();
        assert_ne!(hex_digest_1, hex_digest_2);
    }

    #[test]
    fn changes_the_hash_after_update_with_seed() {
        let mut hash = MurmurHash3_64::new_from_seed(1);
        hash.update(source_text);
        let hex_digest_1 = hash.hexdigest();
        hash.update(source_text);
        let hex_digest_2 = hash.hexdigest();
        assert_ne!(hex_digest_1, hex_digest_2);
    }
}
