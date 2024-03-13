use std::{collections::HashMap, ops::Deref};
use lazy_static::lazy_static;
use std::rc::Rc;
use crate::calc::context::Context;

pub use usize as FunctionId;


/// static function instances
const ALL_FUNCTIONS: [&dyn Functor; 2] = [&Add{}, &Sub{}];


// all function ids, function id must be index of corresponding function in ALL_FUNCTIONS
const ID_ADD: FunctionId = 0;
const ID_SUB: FunctionId = 1;
const ID_MUL: FunctionId = 2;
const ID_DIV: FunctionId = 3;
const ID_MOD: FunctionId = 4;
const ID_POW: FunctionId = 5;
const ID_SQRT: FunctionId = 6;
const ID_ABS: FunctionId = 7;
const ID_NEG: FunctionId = 8;
const ID_SIN: FunctionId = 9;
const ID_COS: FunctionId = 10;
const ID_TAN: FunctionId = 11;
const ID_LN: FunctionId = 12;
const ID_OPEN_BRACKET: FunctionId = 13;
const ID_CLOSE_BRACKET: FunctionId = 14;

const PRIODITY_ADDITIVE: i32 = 6;
const PRIODITY_MULTIPLICATIVE: i32 = 5;
const PRIODITY_USER_FUNCTION: i32 = 2;
const PRIODITY_UNARY_OP: i32 = 3;
/// A trait for a function that can be executed.
pub trait Functor {
    fn execute(&self);
    fn priority(&self) -> i32;
    fn id(&self) -> FunctionId;
}


/// A trait for a function with only one parameter
pub trait UnaryFunctor : Functor {
    fn compute(&self, a: f64) -> f64;
    fn execute(&self) {
        self.compute(0.0);
    }
}

/// A trait for a function with two parameters
pub trait BinaryFunctor {
    fn compute(&self, a: f64, b: f64) -> f64;
    fn execute(&self) {
        Context::with_current(|c| {
            let mut t = c.borrow_mut();
            if t.execution_stack.size() < 2 {
                t.error_detected = true;
                return;
            }
            let b = t.execution_stack.pop_val().unwrap();
            let a = t.execution_stack.pop_val().unwrap();
            let result = self.compute(a, b);
            t.execution_stack.push_val(result);
        });
    }
}

/// Add function
pub struct Add {
}
impl Functor for Add {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_ADD
    }
    fn priority(&self) -> i32 {
        PRIODITY_ADDITIVE
    }
}
impl BinaryFunctor for Add {
    fn compute(&self, a: f64, b: f64) -> f64 {
        a + b
    }
}

/// Sub function
pub struct Sub {
}
impl Functor for Sub {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_SUB
    }
    fn priority(&self) -> i32 {
        PRIODITY_ADDITIVE
    }
}
impl BinaryFunctor for Sub {
    fn compute(&self, a: f64, b: f64) -> f64 {
        a - b
    }
}

/// Mul function
pub struct Mul {
}
impl Functor for Mul {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_MUL
    }
    fn priority(&self) -> i32 {
        PRIODITY_MULTIPLICATIVE
    }
}
impl BinaryFunctor for Mul {
    fn compute(&self, a: f64, b: f64) -> f64 {
        a * b
    }
}

/// Div function
pub struct Div {
}
impl Functor for Div {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> FunctionId {
        ID_DIV
    }
    fn priority(&self) -> i32 {
        PRIODITY_MULTIPLICATIVE
    }
}
impl BinaryFunctor for Div {
    fn compute(&self, a: f64, b: f64) -> f64 {
        a / b
    }
}


type FunctionCreator = fn(&String) -> Box<dyn Functor>;

pub struct FunctionLib {
    function_creator_map: HashMap<String, FunctionCreator>,
}

impl FunctionLib {
    pub fn new() -> Self {
        let mut function_creator_map: HashMap<String, FunctionCreator> = HashMap::new();
        function_creator_map.insert("+".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Add{}) });
        function_creator_map.insert("-".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Sub{}) });
        function_creator_map.insert("*".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Mul{}) });
        function_creator_map.insert("/".to_string(), |_: &String| -> Box<dyn Functor> { Box::new(Div{}) });
        Self {
            function_creator_map
        }        
    }

    pub fn get_functor(&self, name: &String) -> Option<Box<dyn Functor>> {
        self.function_creator_map.get(name).map(|fn_creator| {
            fn_creator(name)
        })
    }
}

lazy_static! {
    pub static ref FUNCTION_LIB: FunctionLib = FunctionLib::new();
}