
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    // default context for a thread
    pub static CURRENT_CONTEXT:  RefCell<Option<Rc<RefCell<Context>>>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct Stack {
    stack_buffer: Rc<RefCell<Vec<f64>>>,
}

#[derive(Clone)]
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
        // CURRENT_CONTEXT.with(|c| {
        //     c.borrow().as_ref().map(|c| c.clone())
        // })

        CURRENT_CONTEXT.with(|c| {
            match c.borrow().as_ref() {
                Some(rc) => {
                    Some(rc.clone())
                },
                None => {
                    None
                }
            }
        })
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack_buffer: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn push_val(&mut self, val: f64) {
        self.stack_buffer.borrow_mut().push(val);
    }

    pub fn pop_val(&mut self) -> Option<f64> {
        self.stack_buffer.borrow_mut().pop()
    }

    pub fn top_val(&self) -> Option<f64> {
        self.stack_buffer.borrow_mut().last().map(|f| *f)
    }

    pub fn size(&self) -> usize {
        self.stack_buffer.borrow().len()
    }
}