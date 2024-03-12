use std::collections::HashMap;

use super::{evaluator, Evaluator};

pub struct Calculator {
    count: u32,
    evaluator: Evaluator,
    constants_map: HashMap<String, String>,
    operand_token: String,
    input_tokens: Vec<String>,
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
            constants_map: HashMap::new(),
            operand_token: String::new(),
            input_tokens: Vec::new(),
        }
    }

    fn expression_operand_input(&mut self, c: &char) -> Result<Option<String>, &str> {        
        self.operand_token.push(*c);
        Ok(Some(self.operand_token.clone()))
    }

    fn expression_token_input(&mut self, op_name: &String) -> Result<Option<String>, &str> {
        if !self.operand_token.is_empty() {
            self.evaluator.put_token(&self.operand_token);
            self.input_tokens.push(self.operand_token.clone());
            self.operand_token.clear()
        }
        let res = self.evaluator.put_token(op_name);
        self.input_tokens.push(op_name.clone());

        match res {
            Err(e) => Err(e),
            Ok(t) => Ok(Some(t.unwrap().to_string().to_string()))
        }
    }

    pub fn build_history(&self) -> String {
        let mut history = String::new();
        for token in &self.input_tokens {
            history.push_str(token);
        }
        history.push_str(&self.operand_token);
        history
    }

    pub fn perform_exp_input(&mut self, input: String) -> Result<Option<String>, &str> {
        if input.is_empty() {
            return Err("Empty input");
        }

        let mut immediate_result: Result<Option<String>, &str>;

        loop {
            if input.len() == 1 {
                let c = input.chars().next().unwrap();
                if c.is_ascii_alphanumeric() || c == '.' {
                    immediate_result = self.expression_operand_input(&c);
                    break;
                }
            }
            let constant = self.constants_map.get(&input);
            match constant {
                Some(value) => {
                    immediate_result = self.expression_token_input(&value.clone());
                }
                None => {
                    immediate_result = self.expression_token_input(&input);
                }
            }
            break;
        }
        
        immediate_result
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