use super::functions::{Functor, FUNCTION_LIB};
use super::context::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_decimal(s : &str) -> bool {
    if s.is_empty() {
        return false;
    }    
    let mut iterator = s.chars();
    let mut oc = iterator.next();
    let mut c = oc.unwrap();    
    if c == '-' || c == '+' {
        if s.len() == 1 {
            return false;
        }
        oc = iterator.next();
    }
    let mut i = 0;
    let mut hasDot = false;
    while oc.is_some() {
        c = oc.unwrap();
        if c == '.' {
            if hasDot { // dot should has only one
                return false;
            }
            hasDot = true;
            if i == 0 { // dot is not allow to be first character
                return false;
            }
        }
        else if !c.is_ascii_digit() {
            return false;
        }
        
        oc = iterator.next();
        i = i + 1;
    }

    true
}

pub struct Evaluator {
    excution_context: Rc<RefCell<Context>>,
    op_stack: Vec<Box<dyn Functor>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            excution_context: Rc::new(RefCell::new(Context::new())),
            op_stack: Vec::new()
        }
    }

    fn push_op(&mut self, op: Box<dyn Functor>) {
        Context::make_current(self.excution_context.clone());
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
        Context::make_current(self.excution_context.clone());
        None
    }

    pub fn put_token(&mut self, token: &String) -> Result<Option<f64>, &str>{
        if token.is_empty() {
            return Err("Empty token");
        }
        if is_decimal(token) {
            return self.put_operand(token);
        }
        
        if token.chars().next().unwrap().is_ascii_digit() {
            // functor is not allow leading by a digit
            return Err("Invalid token");
        }
        
        self.put_functor(token)
    }

    fn put_functor(&mut self, token: &String) -> Result<Option<f64>, &str> {
        FUNCTION_LIB.get_functor(token).map(|functor| {
            match self.top_op() {
                Some(top) => {
                    if functor.priority() < top.priority() {
                        self.push_op(functor);
                        // nothing need to compute then return none
                        None
                    }
                    else {
                        // compute the top functor, the result will be pushed to the stack
                        top.execute();

                        // take away the top functor from the stack due to it is already done
                        self.pop_op();

                        // push the new functor to the stack
                        self.push_op(functor);

                        // read the result from top of the stack then return
                        Some(self.excution_context.borrow().execution_stack.top_val().unwrap().clone())
                    }
                }
                None => {
                    self.push_op(functor);
                    // nothing need to compute then return none
                    None
                }
            }            
        }).ok_or("No functor found")
    }

    fn put_operand(&mut self, token: &String) -> Result<Option<f64>, &str> {        
        token.parse::<f64>().map(|value| {
            self.excution_context.borrow_mut().execution_stack.push_val(value);
            Some(value)
        }).map_err(|_| "Invalid token")
    }

}