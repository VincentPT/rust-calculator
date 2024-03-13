use super::functions::{Functor, FUNCTION_LIB};
use super::context::*;
use std::cell::RefCell;

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
    pub excution_context: RefCell<Context>,
    op_stack: Vec<Box<dyn Functor>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            excution_context: RefCell::new(Context::new()),
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

    pub fn evaluate(&mut self) -> Option<f64> {        
        Context::scope_current(&self.excution_context, |c| {
            while self.op_stack.len() > 0 && c.borrow().error_detected == false {
                let top: Box<dyn Functor> = self.op_stack.pop().unwrap();
                top.execute();
            }
            if c.borrow().execution_stack.size() != 1 || c.borrow().error_detected {
                return None;
            }
            c.borrow().execution_stack.top_val().map(|v| v.clone())
        })
    }

    pub fn put_token(&mut self, token: &String) -> Result<Option<f64>, String>{
        if token.is_empty() {
            return Err("Empty token".to_string());
        }
        if is_decimal(token) {
            return self.put_operand(token);
        }
        
        if token.chars().next().unwrap().is_ascii_digit() {
            // functor is not allow leading by a digit
            return Err("Invalid token".to_string());
        }
        
        self.put_functor(token)
    }

    fn put_functor(&mut self, token: &String) -> Result<Option<f64>, String> {
        let funtor_opt = FUNCTION_LIB.get_functor(token);
        if funtor_opt.is_none() {
            return Err("No functor found".to_string());
        }
        let functor = funtor_opt.unwrap();

        match self.top_op() {
            Some(top) => {
                if functor.priority() < top.priority() {
                    self.push_op(functor);
                    // nothing need to compute then return none
                    Ok(None)
                }
                else {
                    // compute the top functor, the result will be pushed to the stack
                    Context::scope_current(&self.excution_context, |_| {
                        // execute function require an execution context.
                        // So, we must ensure it will be use the current context in stead of the default one.
                        // By bounding it by using the scope_current function, we can ensure the current context will be used.
                        top.execute();
                    });
                    if self.excution_context.borrow().error_detected {
                        if self.excution_context.borrow().error_message.is_empty() {
                            Err("Error".to_string())
                        }
                        else {
                            return Err(self.excution_context.borrow().error_message.clone())
                        }
                    }
                    else {
                        // take away the top functor from the stack due to it is already done
                        self.pop_op();

                        // push the new functor to the stack
                        self.push_op(functor);

                        // read the result from top of the stack then return
                        Ok(Some(self.excution_context.borrow().execution_stack.top_val().unwrap().clone()))
                    }
                }
            }
            None => {
                self.push_op(functor);
                // nothing need to compute then return none
                Ok(None)
            }
        }
    }

    fn put_operand(&mut self, token: &String) -> Result<Option<f64>, String> {        
        token.parse::<f64>().map(|value| {
            self.excution_context.borrow_mut().execution_stack.push_val(value);
            Some(value)
        }).map_err(|_| "Invalid token".to_string())
    }

}