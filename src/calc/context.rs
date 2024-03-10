
use std::cell::RefCell;

use super::functions::Functor;
use super::functions::FunctionId;

thread_local! {
    static CURRENT_CONTEXT: Context = Context { execution_stack: Stack::new() };
}


pub struct Stack {
    stack_buffer: Vec<i32>,
}

pub struct Context {
    pub execution_stack: Stack,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack_buffer: Vec::new(),
        }
    }

    pub fn push_val(&mut self, val: i32) {
        self.stack_buffer.push(val);
    }

    pub fn pop_val(&mut self) -> Option<i32> {
        self.stack_buffer.pop()
    }

    pub fn top_val(&self) -> Option<&i32> {
        self.stack_buffer.last()
    }

    pub fn size(&self) -> usize {
        self.stack_buffer.len()
    }
}