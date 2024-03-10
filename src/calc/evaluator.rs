use super::functions::Functor;

pub struct Evaluator {
    pub count: u32,
    op_stack: Vec<Box<dyn Functor>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            count: 0,
            op_stack: Vec::new()
        }
    }

    fn push_op(&mut self, op: Box<dyn Functor>) {
        self.op_stack.push(op);
    }

    fn pop_op(&mut self) {
        self.op_stack.pop();
    }

    fn top_op(&self) -> Option<& Box<dyn Functor>> {
        self.op_stack.last()
    }

    pub fn op_size(&self) -> usize {
        self.op_stack.len()
    }

    pub fn evaluate(&mut self, input: &String) -> Option<f64> {
        None
    }

    pub fn put_token(&mut self, token: &String) -> Option<f64> {
        None
    }
}