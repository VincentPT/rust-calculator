use super::{evaluator, Evaluator};

pub struct Calculator {
    pub count: u32,
    pub evaluator: Evaluator<'static>,
}
pub enum Feature {
    CE,
    C,
    MS,
    MR,
    DEL,
    Eval,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            count: 0,
            evaluator: Evaluator::new(),
        }
    }

    pub fn perform_exp_input(&mut self, input: String) -> (Option<String>, Option<String>) {
        self.count += 1;
        let history = self.count.to_string();
        self.evaluator.evaluate(input.as_str());
        (Some(history), Some(input))
    }

    pub fn perform_feature(&mut self, feature: &Feature) -> (Option<String>, Option<String>) {
        let history = self.count.to_string();
        let value = match feature {
            Feature::CE => "CE".to_string(),
            Feature::C => "C".to_string(),
            Feature::MS => "MS".to_string(),
            Feature::MR => "MR".to_string(), 
            Feature::Eval => "Eval".to_string(),
            Feature::DEL => "DEL".to_string(),
        };
        (Some(history), Some(value))
    }
}