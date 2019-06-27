use lazy_static::*;
use num_derive::{FromPrimitive, ToPrimitive};
use snafu::*;
use std::sync::Mutex;

#[cfg(target_arch = "wasm32")]
use js_sys::{JsString, Reflect, Uint8Array};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum VerbosityLevel {
    Errors = 0,
    Warnings = 1,
    Infos = 5,
}

#[derive(Debug, Snafu)]
pub enum VerbosityLevelError {
    #[snafu(display("could not obtain lock for verbosity level"))]
    CouldNotObtainLock,
}

lazy_static! {
    static ref VERBOSITY_LEVEL: Mutex<VerbosityLevel> = Mutex::new(VerbosityLevel::Warnings);
}

pub fn set_verbosity_level(level: VerbosityLevel) -> Result<(), VerbosityLevelError> {
    *VERBOSITY_LEVEL
        .lock()
        .map_err(|_| VerbosityLevelError::CouldNotObtainLock)? = level;
    Ok(())
}

pub fn verbosity_level() -> Result<VerbosityLevel, VerbosityLevelError> {
    Ok(*VERBOSITY_LEVEL
        .lock()
        .map_err(|_| VerbosityLevelError::CouldNotObtainLock)?)
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
#[wasm_bindgen(js_name = stringToBytes)]
pub fn string_to_bytes(s: JsValue) -> Result<Uint8Array, JsValue> {
    if let Ok(s) = s.dyn_into::<JsString>() {
        let length = s.length();
        let bytes = Uint8Array::new(&length.into());
        for i in 0..length {
            Reflect::set_u32(&bytes, i, &s.char_code_at(i).into())
                .expect("could not write to bytes");
        }
        Ok(bytes)
    } else {
        Err(js_sys::Error::new("Invalid argument for stringToBytes").into())
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use js_sys::Error;
    use num_traits::{FromPrimitive, ToPrimitive};

    #[wasm_bindgen(js_name = setVerbosityLevel)]
    pub fn set_verbosity_level(level: u8) -> Result<(), JsValue> {
        VerbosityLevel::from_u8(level)
            .ok_or(Error::new("could not convert integer to verbosity level").into())
            .and_then(|level| {
                super::set_verbosity_level(level).map_err(|e| Error::new(&e.to_string()).into())
            })
    }

    #[wasm_bindgen(js_name = getVerbosityLevel)]
    pub fn verbosity_level() -> Result<u8, JsValue> {
        super::verbosity_level()
            .map_err(|e: VerbosityLevelError| JsValue::from(js_sys::Error::new(&e.to_string())))?
            .to_u8()
            .ok_or(js_sys::Error::new("could not convert VerbosityLevel to u8").into())
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
