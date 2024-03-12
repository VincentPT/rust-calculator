
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    // default context for a thread
    pub static CURRENT_CONTEXT:  RefCell<Option<Rc<RefCell<Context>>>> = RefCell::new(None);
}


pub struct Stack {
    stack_buffer: Vec<f64>,
}

pub struct Context {
    pub execution_stack: Stack,
}

impl Context {
    pub fn new() -> Self {
        Self {
            execution_stack: Stack::new(),
        }
    }
    pub fn make_current(context: Rc<RefCell<Context>>) {
        CURRENT_CONTEXT.with(|c| {
            *c.borrow_mut() = Some(context);
        });
    }

    pub fn get_current() -> Option<Rc<RefCell<Context>>> {
        CURRENT_CONTEXT.with(|c| {
            c.borrow().as_ref().map(|c| c.clone())
        })
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack_buffer: Vec::new(),
        }
    }

    pub fn push_val(&mut self, val: f64) {
        self.stack_buffer.push(val);
    }

    pub fn pop_val(&mut self) -> Option<f64> {
        self.stack_buffer.pop()
    }

    pub fn top_val(&self) -> Option<&f64> {
        self.stack_buffer.last()
    }

    pub fn size(&self) -> usize {
        self.stack_buffer.len()
    }
}