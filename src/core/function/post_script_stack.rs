use snafu::*;
use js_sys::Number;

#[derive(Debug, Snafu, Copy, Clone, Eq, PartialEq)]
pub enum PostScriptStackError {
    #[snafu(display("PostScript function stack overflow."))]
    StackOverflow,
    #[snafu(display("PostScript function stack underflow."))]
    StackUnderflow
}

const MAX_STACK_SIZE: usize = 100;

pub struct PostScriptStack {
    stack: Vec<Number>
}

impl PostScriptStack {
    pub fn new() -> Self {
        PostScriptStack {
            stack: Vec::new()
        }
    }

    pub fn new_from_initial_stack(initial_stack: Vec<Number>) -> Self {
        PostScriptStack {
            stack: initial_stack
        }
    }

    pub fn push(&mut self, value: Number) -> Result<(), PostScriptStackError> {
        if self.stack.len() >= MAX_STACK_SIZE {
            Err(PostScriptStackError::StackOverflow)
        } else {
            self.stack.push(value);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<Number, PostScriptStackError> {
        self.stack.pop().ok_or(PostScriptStackError::StackUnderflow)
    }

    pub fn copy(&mut self, n: usize) -> Result<(), PostScriptStackError> {
        if self.stack.len() + n >= MAX_STACK_SIZE {
            Err(PostScriptStackError::StackOverflow)
        } else {
            let stack_len = self.stack.len();
            for i in (stack_len - n)..stack_len {
                self.stack.push(self.stack[i].clone());
            }
            Ok(())
        }
    }

    pub fn index(&mut self, n: usize) -> Result<(), PostScriptStackError> {
        self.push(self.stack[self.stack.len() - n - 1].clone())
    }

    pub fn roll(&mut self, n: usize, p: usize) {
        let l = self.stack.len() - n;
        let r = self.stack.len() - 1;
        let c = l + (p - (p / n) * n);
        let mut i;
        let mut j;
        let mut t;
        i = l;
        j = r;
        loop {
            if i >= j {
                break;
            }
            t = self.stack[i].clone();
            self.stack[i] = self.stack[j].clone();
            self.stack[j] = t;
            i += 1;
            j -= 1;
        }
        i = l;
        j = c - 1;
        loop {
            if i >= j {
                break;
            }
            t = self.stack[i].clone();
            self.stack[i] = self.stack[j].clone();
            self.stack[j] = t;
            i += 1;
            j -= 1;
        }
        i = c;
        j = r;
        loop {
            if i >= j {
                break;
            }
            t = self.stack[i].clone();
            self.stack[i] = self.stack[j].clone();
            self.stack[j] = t;
            i += 1;
            j -= 1;
        }
    }
}
