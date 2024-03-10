use super::Context;
use super::functions::FUNCTIONS;

pub struct Evaluator<'a> {
    pub count: u32,
    context: Context<'a>,
}

impl<'a> Evaluator<'a> {
    pub fn new() -> Self {
        Self {
            count: 0,
            context: Context::new(),
        }
    }

    pub fn evaluate(&mut self, input: &str) -> (Option<String>, Option<String>) {
        let f = FUNCTIONS.get_functor();
        match f {
            Some(functor) => {
                self.context.push_op(functor);
            }
            None => {}
        };

        match self.context.top_op() {
            Some(op) => {
                op.execute();
            }
            None => {}
        }; 
        self.count += 1;
        let history = self.count.to_string();
        let value = input.to_string();
        (Some(history), Some(value))
    }
}