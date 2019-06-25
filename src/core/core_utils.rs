#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Casing {
    Upper,
    Lower,
}

const ROMAN_NUMBER_MAP_HUNDREDS: [&str; 10] =
    ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];

const ROMAN_NUMBER_MAP_TENS: [&str; 10] =
    ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];

const ROMAN_NUMBER_MAP_ONES: [&str; 10] =
    ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

/// Converts positive integers to (upper case) Roman numerals.
/// - `number`: The number that should be converted.
/// - `casing`: Indicates the casing the result should have.
pub fn to_roman_numerals(number: u32, casing: Casing) -> String {
    use core::iter::once;

    let thousands = number / 1000;
    let hundreds = (number / 100) % 10;
    let tens = (number / 10) % 10;
    let ones = number % 10;

    let upper_case = once("M")
        .cycle()
        .take(thousands as usize)
        .chain(once(ROMAN_NUMBER_MAP_HUNDREDS[hundreds as usize]))
        .chain(once(ROMAN_NUMBER_MAP_TENS[tens as usize]))
        .chain(once(ROMAN_NUMBER_MAP_ONES[ones as usize]))
        .collect();

    if casing == Casing::Upper {
        upper_case
    } else {
        upper_case.to_lowercase()
    }
}

#[test]
fn test_to_roman_numerals_uppercase() {
    assert_eq!(to_roman_numerals(1, Casing::Upper), "I");
    assert_eq!(to_roman_numerals(6, Casing::Upper), "VI");
    assert_eq!(to_roman_numerals(7, Casing::Upper), "VII");
    assert_eq!(to_roman_numerals(8, Casing::Upper), "VIII");
    assert_eq!(to_roman_numerals(10, Casing::Upper), "X");
    assert_eq!(to_roman_numerals(40, Casing::Upper), "XL");
    assert_eq!(to_roman_numerals(100, Casing::Upper), "C");
    assert_eq!(to_roman_numerals(500, Casing::Upper), "D");
    assert_eq!(to_roman_numerals(1000, Casing::Upper), "M");
    assert_eq!(to_roman_numerals(2019, Casing::Upper), "MMXIX");
}

#[test]
fn test_to_roman_numerals_lowercase() {
    assert_eq!(to_roman_numerals(1, Casing::Lower), "i");
    assert_eq!(to_roman_numerals(6, Casing::Lower), "vi");
    assert_eq!(to_roman_numerals(7, Casing::Lower), "vii");
    assert_eq!(to_roman_numerals(8, Casing::Lower), "viii");
    assert_eq!(to_roman_numerals(10, Casing::Lower), "x");
    assert_eq!(to_roman_numerals(40, Casing::Lower), "xl");
    assert_eq!(to_roman_numerals(100, Casing::Lower), "c");
    assert_eq!(to_roman_numerals(500, Casing::Lower), "d");
    assert_eq!(to_roman_numerals(1000, Casing::Lower), "m");
    assert_eq!(to_roman_numerals(2019, Casing::Lower), "mmxix");
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use js_sys::{Error, Number};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen(js_name = toRomanNumerals)]
    pub fn to_roman_numerals(value: JsValue, lowercase: Option<bool>) -> Result<String, JsValue> {
        let number = Some(value)
            .filter(Number::is_integer)
            .and_then(|f| f.as_f64())
            .filter(|f| *f > 0.0)
            .map(|f| f as u32)
            .ok_or(JsValue::from(Error::new(
                "The number should be a positive integer.",
            )))?;

        let casing = if lowercase.unwrap_or(false) {
            Casing::Lower
        } else {
            Casing::Upper
        };
        Ok(super::to_roman_numerals(number, casing))
    }

    #[wasm_bindgen_test]
    fn test_to_roman_numerals_uppercase() {
        assert_eq!(to_roman_numerals(1.into(), None), Ok("I".to_string()));
        assert_eq!(to_roman_numerals(6.into(), None), Ok("VI".to_string()));
        assert_eq!(to_roman_numerals(7.into(), None), Ok("VII".to_string()));
        assert_eq!(to_roman_numerals(8.into(), None), Ok("VIII".to_string()));
        assert_eq!(to_roman_numerals(10.into(), None), Ok("X".to_string()));
        assert_eq!(to_roman_numerals(40.into(), None), Ok("XL".to_string()));
        assert_eq!(to_roman_numerals(100.into(), None), Ok("C".to_string()));
        assert_eq!(to_roman_numerals(500.into(), None), Ok("D".to_string()));
        assert_eq!(to_roman_numerals(1000.into(), None), Ok("M".to_string()));
        assert_eq!(
            to_roman_numerals(2019.into(), None),
            Ok("MMXIX".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn test_to_roman_numerals_lowercase() {
        assert_eq!(to_roman_numerals(1.into(), Some(true)), Ok("i".to_string()));
        assert_eq!(
            to_roman_numerals(6.into(), Some(true)),
            Ok("vi".to_string())
        );
        assert_eq!(
            to_roman_numerals(7.into(), Some(true)),
            Ok("vii".to_string())
        );
        assert_eq!(
            to_roman_numerals(8.into(), Some(true)),
            Ok("viii".to_string())
        );
        assert_eq!(
            to_roman_numerals(10.into(), Some(true)),
            Ok("x".to_string())
        );
        assert_eq!(
            to_roman_numerals(40.into(), Some(true)),
            Ok("xl".to_string())
        );
        assert_eq!(
            to_roman_numerals(100.into(), Some(true)),
            Ok("c".to_string())
        );
        assert_eq!(
            to_roman_numerals(500.into(), Some(true)),
            Ok("d".to_string())
        );
        assert_eq!(
            to_roman_numerals(1000.into(), Some(true)),
            Ok("m".to_string())
        );
        assert_eq!(
            to_roman_numerals(2019.into(), Some(true)),
            Ok("mmxix".to_string())
        );
    }
}
