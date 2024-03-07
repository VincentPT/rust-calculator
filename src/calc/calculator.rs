trait CalculatorView {
    fn set_result(&self, result: String);
    fn set_history(&self, history: String);
}
pub struct Calculator {
    pub value: f64,
    //pub view: Rc<>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            //view: None
        }
    }
}