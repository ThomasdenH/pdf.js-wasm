use lazy_static::*;
use num_derive::{FromPrimitive, ToPrimitive};
use std::sync::Mutex;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum VerbosityLevel {
    Errors = 0,
    Warnings = 1,
    Infos = 5,
}

lazy_static! {
    static ref VERBOSITY_LEVEL: Mutex<VerbosityLevel> = Mutex::new(VerbosityLevel::Warnings);
}

pub fn set_verbosity_level(level: VerbosityLevel) {
    *VERBOSITY_LEVEL.lock().unwrap() = level;
}

pub fn verbosity_level() -> VerbosityLevel {
    *VERBOSITY_LEVEL.lock().unwrap()
}

/// A notice for devs. These are good for things that are helpful to devs, such
/// as warning that Workers were disabled, which is important to devs but not
/// end users.
pub fn info(msg: &str) {
    println!("Info: {}", msg);
}

/// Non-fatal warnings.
pub fn warn(msg: &str) {
    println!("Warning: {}", msg);
}

/// Checks if ch is one of the following characters: SPACE, TAB, CR or LF.
pub fn is_space(ch: char) -> bool {
    ch == 0x20.into() || ch == 0x09.into() || ch == 0x0D.into() || ch == 0x0A.into()
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use num_traits::{FromPrimitive, ToPrimitive};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = setVerbosityLevel)]
    pub fn set_verbosity_level(level: u8) {
        super::set_verbosity_level(VerbosityLevel::from_u8(level).unwrap())
    }

    #[wasm_bindgen(js_name = getVerbosityLevel)]
    pub fn verbosity_level() -> u8 {
        super::verbosity_level().to_u8().unwrap()
    }

    #[wasm_bindgen]
    pub fn warn(msg: &str) {
        super::warn(msg)
    }

    #[wasm_bindgen]
    pub fn info(msg: &str) {
        super::info(msg)
    }

    #[wasm_bindgen(js_name = isSpace)]
    pub fn is_space(ch: u8) -> bool {
        super::is_space(ch.into())
    }
}
