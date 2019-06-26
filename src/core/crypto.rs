#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct ArcFourCipher {
    a: u8,
    b: u8,
    s: Vec<u8>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ArcFourCipher {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(key: &[u8]) -> ArcFourCipher {
        let a = 0u8;
        let b = 0u8;
        let mut j = 0u8;
        let mut s: Vec<u8> = (0u8..=255).into_iter().collect();
        for i in 0..256 {
            let tmp = s[i];
            j = j.wrapping_add(tmp).wrapping_add(key[i % key.len()]);
            s[i] = s[usize::from(j)];
            s[usize::from(j)] = tmp;
        }
        ArcFourCipher { a, b, s }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name=encryptBlock))]
    pub fn encrypt_block(&mut self, data: &[u8]) -> Vec<u8> {
        (0..data.len())
            .into_iter()
            .map(|i| {
                self.a = self.a.wrapping_add(1);
                let tmp = self.s[usize::from(self.a)];
                self.b = self.b.wrapping_add(tmp);
                let tmp2 = self.s[usize::from(self.b)];
                self.s[usize::from(self.a)] = tmp2;
                self.s[usize::from(self.b)] = tmp;
                data[i] ^ self.s[usize::from(tmp.wrapping_add(tmp2))]
            })
            .collect()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name=decryptBlock))]
    pub fn decrypt_block(&mut self, data: &[u8]) -> Vec<u8> {
        self.encrypt_block(data)
    }
}
