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

#[cfg(feature = "wasm")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = toRomanNumerals)]
    pub fn to_roman_numerals(number: u32, lowercase: Option<bool>) -> String {
        let casing = if lowercase.unwrap_or(false) {
            Casing::Lower
        } else {
            Casing::Upper
        };
        super::to_roman_numerals(number, casing)
    }
}
