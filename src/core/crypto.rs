use md5;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

mod arc_four_cipher;

pub use arc_four_cipher::ArcFourCipher;

#[cfg(target_arch = "wasm32")]
fn uint8_array_to_vec(u: Uint8Array) -> Vec<u8> {
  let mut vec: Vec<u8> = (0..u.length()).into_iter().map(|_| 0).collect();
  u.copy_to(&mut vec);
  vec
}

pub fn calculate_md5(data: impl AsRef<[u8]>, offset: usize, length: usize) -> [u8; 16] {
  md5::compute(&data.as_ref()[offset..(offset + length)]).into()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = calculateMD5)]
pub fn calculate_md5_wasm(
  data: JsValue,
  offset: JsValue,
  length: JsValue,
) -> Result<Uint8Array, JsValue> {
  let length_u32 = length.as_f64().ok_or("expected length to be a number")?;
  let offset_u32 = offset.as_f64().ok_or("expected offset to be a number")?;
  let data_vec: Vec<u8> = uint8_array_to_vec(data.dyn_into()?);
  let result = calculate_md5(&data_vec, offset_u32 as usize, length_u32 as usize).to_vec();
  // Use Into when it is available
  Ok(unsafe { Uint8Array::new(&Uint8Array::view(&result)) })
}

#[cfg(test)]
mod test {
  use hex;

  fn assert_md5_equal(data: impl AsRef<[u8]>, hex_str: &str) {
    let expected = hex::decode(hex_str).expect("provided hex_str not valid hex");
    let result = super::calculate_md5(data.as_ref(), 0, data.as_ref().len()).to_vec();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_pass_rfc_1321_test_1() {
    assert_md5_equal(b"", "d41d8cd98f00b204e9800998ecf8427e");
  }

  #[test]
  fn should_pass_rfc_1321_test_2() {
    assert_md5_equal(b"a", "0cc175b9c0f1b6a831c399e269772661");
  }

  #[test]
  fn should_pass_rfc_1321_test_3() {
    assert_md5_equal(b"abc", "900150983cd24fb0d6963f7d28e17f72");
  }

  #[test]
  fn should_pass_rfc_1321_test_4() {
    assert_md5_equal(b"message digest", "f96b697d7cb7938d525a2f31aaf161d0");
  }

  #[test]
  fn should_pass_rfc_1321_test_5() {
    assert_md5_equal(
      b"abcdefghijklmnopqrstuvwxyz",
      "c3fcd3d76192e4007dfb496cca67e13b",
    );
  }

  #[test]
  fn should_pass_rfc_1321_test_6() {
    assert_md5_equal(
      b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec(),
      "d174ab98d277d9f5a5611c2c9f419d9f",
    );
  }

  #[test]
  fn should_pass_rfc_1321_test_7() {
    assert_md5_equal(
      b"12345678901234567890123456789012345678901234567890123456789012345678901234567890".to_vec(),
      "57edf4a22be3c955ac49da2e2107b67a",
    );
  }

  #[cfg(target_arch = "wasm32")]
  mod wasm {
    use js_sys::{Error, JsString, Uint8Array};
    fn hex_2_binary(s: JsString) -> Result<Uint8Array, Error> {
      Ok(unsafe {
        Uint8Array::new(&Uint8Array::view(
          &hex::decode(s.as_string().ok_or(Error::new(
            "expected argument of hex_2_binary to be a string",
          ))?)
          .map_err(|_| Error::new("expected argument of hex_2_binary to be valid hex"))?,
        ))
      })
    }

    mod calculate_md5 {
      use super::super::super::calculate_md5_wasm;
      use super::hex_2_binary;
      use crate::shared::util::string_to_bytes;
      use js_sys::Uint8Array;
      use wasm_bindgen_test::*;

      fn assert_uint8arrays_eq(a: Uint8Array, b: Uint8Array) {
        assert_eq!(
          dbg!(super::super::super::uint8_array_to_vec(a)),
          dbg!(super::super::super::uint8_array_to_vec(b))
        );
      }

      #[wasm_bindgen_test]
      fn should_pass_rfc_1321_test_1() {
        let input = string_to_bytes("".into()).unwrap();
        let length = input.length().into();
        let result = calculate_md5_wasm(input.into(), 0.into(), length);
        let expected = hex_2_binary("d41d8cd98f00b204e9800998ecf8427e".into());
        assert_uint8arrays_eq(result.unwrap(), expected.unwrap());
      }
    }
  }
}
