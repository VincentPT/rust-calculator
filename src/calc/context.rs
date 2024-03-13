
use std::cell::RefCell;

thread_local! {
    // default context for a thread    
    pub static CURRENT_CONTEXT: RefCell<Context> = RefCell::new(Context::new());
}

pub struct Stack {
    stack_buffer: Vec<f64>,
}
pub struct Context {
    pub execution_stack: Stack,
    pub error_detected: bool,
    pub error_message: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            execution_stack: Stack::new(),
            error_detected: false,
            error_message: String::new(),
        }
    }
    pub fn scope_current<F, T>(context:& RefCell<Context>, scope: F) -> T
    where F: FnOnce(&RefCell<Context>) -> T {
        CURRENT_CONTEXT.with(|c| {
            c.swap(context);

            let t = scope(c);

            context.swap(c);
            // return
            t
        })
    }

    pub fn with_current<F>(f: F)
    where F: FnOnce(&RefCell<Context>)
    {
        CURRENT_CONTEXT.with(|c| {
            f(c);
        });
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