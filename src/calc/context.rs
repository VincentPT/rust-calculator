use super::functions::{Functor};


pub struct Context<'a> {
    op_stack: Vec<& 'a dyn Functor>,
    val_stack: Vec<i32>,
}

impl <'a> Context<'a>{
    pub fn new() -> Self {
        Self {
            op_stack: Vec::new(),
            val_stack: vec![0],
        }
    }

    pub fn push_op(&mut self, op: & 'a dyn Functor) {
        self.op_stack.push(op);
    }

    pub fn pop_op(&mut self) {
        self.op_stack.pop();
    }

    pub fn top_op(&self) -> Option<& & 'a dyn Functor> {
        self.op_stack.last()
    }

    pub fn push_val(&mut self, val: i32) {
        self.val_stack.push(val);
    }

    pub fn pop_val(&mut self) {
        self.val_stack.pop();
    }

    pub fn top_val(&self) -> Option<&i32> {
        self.val_stack.last()
    }

    pub fn op_size(&self) -> usize {
        self.op_stack.len()
    }

    pub fn val_size(&self) -> usize {
        self.val_stack.len()
    }
}