use std::collections::HashMap;
use lazy_static::lazy_static;


/// static function instances
const ALL_FUNCTIONS: [&dyn Functor; 2] = [&Add{}, &Sub{}];

// all function ids, function id must be index of corresponding function in ALL_FUNCTIONS
const ID_ADD: usize = 0;
const ID_SUB: usize = 1;
const ID_MUL: usize = 2;
const ID_DIV: usize = 3;
const ID_MOD: usize = 4;
const ID_POW: usize = 5;
const ID_SQRT: usize = 6;
const ID_ABS: usize = 7;
const ID_NEG: usize = 8;
const ID_SIN: usize = 9;
const ID_COS: usize = 10;
const ID_TAN: usize = 11;
const ID_LN: usize = 12;
const ID_OPEN_BRACKET: usize = 13;
const ID_CLOSE_BRACKET: usize = 14;

const PRIODITY_ADDITIVE: i32 = 6;
const PRIODITY_MULTIPLICATIVE: i32 = 5;
const PRIODITY_USER_FUNCTION: i32 = 2;
const PRIODITY_UNARY_OP: i32 = 3;
/// A trait for a function that can be executed.
pub trait Functor {
    fn execute(&self);
    fn priority(&self) -> i32;
    fn id(&self) -> usize;
}


/// A trait for a function with only one parameter
pub trait UnaryFunctor : Functor {
    fn compute(&self, a: i32) -> i32;
    fn execute(&self) {
        self.compute(0);
    }
}

/// A trait for a function with two parameters
pub trait BinaryFunctor {
    fn compute(&self, a: i32, b: i32) -> i32;
    fn execute(&self) {
        self.compute(1, 2);
    }
}

/// Add function
pub struct Add {
}
impl Functor for Add {
    fn execute(&self) {
        BinaryFunctor::execute(self);
    }
    fn id(&self) -> usize {
        ID_ADD
    }
    fn priority(&self) -> i32 {
        PRIODITY_ADDITIVE
    }
}
impl BinaryFunctor for Add {
    fn compute(&self, a: i32, b: i32) -> i32 {
        println!("{} + {} = {}", a, b, a + b);
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
    fn id(&self) -> usize {
        ID_SUB
    }
    fn priority(&self) -> i32 {
        PRIODITY_ADDITIVE
    }
}
impl BinaryFunctor for Sub {
    fn compute(&self, a: i32, b: i32) -> i32 {
        println!("{} - {} = {}", a, b, a - b);
        a - b
    }
}

pub struct Functions {
    function_idx_map: HashMap<String, usize>,
    // pub sub: Sub,
    // pub mul: Mul,
    // pub div: Div,
}

impl Functions {
    pub fn new() -> Self {
        let mut function_idx_map: HashMap<String, usize> = HashMap::new();
        function_idx_map.insert("+".to_string(), ID_ADD);
        Self {
            function_idx_map,
        }
    }

    pub fn get_functor(&self) -> Option<&dyn Functor> {
        match self.function_idx_map.get("+") {
            Some(id) => {
                Some(ALL_FUNCTIONS[*id])
            }
            None => {
                None
            }
        }
    }
}

lazy_static! {
    pub static ref FUNCTIONS: Functions = Functions::new();
}

// fn get_function() -> & 'static Functions {
//     let f = &FUNCTIONS;
//     return f;
// }