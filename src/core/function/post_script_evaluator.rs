use js_sys::{Number, Math};
use wasm_bindgen::JsValue;
use crate::core::function::{PostScriptStack, PostScriptStackError};
use snafu::*;

#[derive(Eq, PartialEq, Clone, Copy, Eq, PartialEq, Error)]
pub enum PostScriptEvaluatorError {
    #[snafu(display("unknown operator {}", operator))]
    Format { operator: JsValue },
    #[snafu(display("error pushing operand: {}", source))]
    PushOperand { source: PostScriptStackError }
}

pub struct PostScriptEvaluator {
    operators: Vec<JsValue>
}

impl PostScriptEvaluator {
    pub fn new(operators: Vec<JsValue>) -> Self {
        PostScriptEvaluator {
            operators
        }
    }

    pub fn execute(initial_stack: Vec<Number>) -> Result<PostScriptStack, PostScriptEvaluatorError> {
        let mut stack = PostScriptStack::new(initial_stack);
        let mut counter = 0;
        while counter < self.operators.len() {
            let operator = self.operators[counter];
            counter += 1;
            if let Ok(number) = operator.dyn_into::<Number>() {
                stack.push(number).context(PostScriptEvaluatorError::PushOperand {})?;
                continue;
            } else if let Some(s) = operator.as_string() {
                match s {
                    // non standard ps operators
                    "jz" => { // jump if false
                        let b = stack.pop();
                        let a = stack.pop();
                        if a == 0u32 {
                            counter = b.as_f64().unwrap() as usize;
                        }
                    },
                    "j" => { // jump
                        let a = stack.pop()
                        counter = a.as_f64().unwarp() as usize
                    },

                    // all ps operators in alphabetical order (excluding if/ifelse)
                    "abs" => {
                        let a = stack.pop();
                        stack.push(Math::abs(a));
                    },
                    "add" => {
                        
                    }
                    _ => unimplemented!();
                }
            } else {
                return PostScriptEvaluatorError::Format { operator };
            }
        }
        Ok(stack)
    }
}
