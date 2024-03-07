use std::rc::{Rc, Weak};

pub trait CalculatorView {
    fn set_result(self: &mut Self, result: String);
    fn set_history(self: &mut Self, history: String);
}
pub struct Calculator<'a, TView> {
    pub value: f64,
    pub view: &'a mut TView,
}


impl<'a, TView> Calculator<'a, TView> {
    pub fn new(view: &'a mut TView) -> Self {
        Self {
            value: 0.0,
            view,
        }
    }
}

impl<'a, TView> Calculator<'a, TView> where TView:CalculatorView, {
    pub fn do_something(self: &mut Self) {
        self.view.set_result("Hello".to_string());
    }
}

pub struct TestCalculator<TView> {
    pub value: f64,
    pub view: Rc<TView>,
}


impl<TView> TestCalculator<TView> {
    pub fn new(view: Rc<TView>) -> Self {
        Self {
            value: 0.0,
            view,
        }
    }
}

impl<TView> TestCalculator<TView> where TView:CalculatorView, {
    pub fn do_something(&mut self) {
        let view = Rc::get_mut(&mut self.view).unwrap();
        view.set_result("Hello".to_string());    
    }
}


pub struct TestCalculator2<TView> {
    pub value: f64,
    pub view: Weak<TView>,
}


impl<TView> TestCalculator2<TView> {
    pub fn new(view: &Weak<TView>) -> Self {
        Self {
            value: 0.0,
            view : view.clone(),
        }
    }
}