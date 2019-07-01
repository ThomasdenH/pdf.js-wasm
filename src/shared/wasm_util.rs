use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::Number;

pub trait IsTruthy {
    fn is_falsy(&self) -> bool;
    fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }
}

impl IsTruthy for JsValue {
    fn is_falsy(&self) -> bool {
        self.as_bool() == Some(false) ||
        self.as_string().map(|s| s.is_empty()).unwrap_or(false) ||
        self.is_null() ||
        self.is_undefined() ||
        self.dyn_ref::<Number>()
            .map(|number| number == &0u32 || Number::is_nan(&number))
            .unwrap_or(false)
    }
}

#[wasm_bindgen_test]
fn test_is_truthy() {
    assert_eq!(JsValue::from(0).is_truthy(), false);
    assert_eq!(JsValue::from(0).is_falsy(), true);
    assert_eq!(JsValue::from("".to_string()).is_truthy(), false);
    assert_eq!(JsValue::from("".to_string()).is_falsy(), true);
    assert_eq!(JsValue::from(false).is_truthy(), false);
    assert_eq!(JsValue::from(false).is_falsy(), true);
    assert_eq!(JsValue::NULL.is_truthy(), false);
    assert_eq!(JsValue::NULL.is_falsy(), true);
    assert_eq!(JsValue::UNDEFINED.is_truthy(), false);
    assert_eq!(JsValue::UNDEFINED.is_falsy(), true);

    assert_eq!(JsValue::from(10).is_truthy(), true);
    assert_eq!(JsValue::from(10).is_falsy(), false);
    assert_eq!(JsValue::from("null".to_string()).is_truthy(), true);
    assert_eq!(JsValue::from("null".to_string()).is_falsy(), false);
    assert_eq!(JsValue::from(true).is_truthy(), true);
    assert_eq!(JsValue::from(true).is_falsy(), false);
}
