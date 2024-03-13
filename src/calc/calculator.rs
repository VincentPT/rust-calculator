use std::collections::HashMap;

use super::Evaluator;

pub struct Calculator {
    evaluator: Evaluator,
    constants_map: HashMap<String, String>,
    operand_token: String,
    last_result: String,
    temp_history: String,
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
            evaluator: Evaluator::new(),
            constants_map: HashMap::new(),
            operand_token: String::new(),
            input_tokens: Vec::new(),
            last_result: "0".to_string(),
            temp_history: String::new(),
        }
    }

    fn expression_operand_input(&mut self, c: &char) -> Result<Option<String>, &str> {       
        if !self.last_result.is_empty() {
            // clear last result if user input first operand of the expression
            self.last_result.clear();
        }
        self.operand_token.push(*c);
        Ok(Some(self.operand_token.clone()))
    }

    fn expression_constant_input(&mut self, const_val: &String) -> Result<Option<String>, &str> {
        // just clear the temporary input if user pick another constant
        self.operand_token.clear();
        // clear last result we don't need it anymore
        self.last_result.clear();

        let res = self.evaluator.put_token(const_val);
        self.input_tokens.push(const_val.clone());

        match res {
            Err(e) => Err(e),
            Ok(t) => {
                Ok(t.map(|v| v.to_string()))
            }
        }
    }

    fn expression_op_input(&mut self, op_name: &String) -> Result<Option<String>, &str> {
        if !self.last_result.is_empty() {
            self.operand_token = self.last_result.clone();
            self.last_result.clear();
        }
        if !self.operand_token.is_empty() {
            let _ = self.evaluator.put_token(&self.operand_token);
            self.input_tokens.push(self.operand_token.clone());
            self.operand_token.clear()
        }
        let res = self.evaluator.put_token(op_name);
        self.input_tokens.push(op_name.clone());

        match res {
            Err(e) => Err(e),
            Ok(t) => {
                Ok(t.map(|v| v.to_string()))
            }
        }
    }

    pub fn build_history(&self) -> String {
        if self.temp_history.is_empty() {
            let mut history = String::new();
            for token in &self.input_tokens {
                history.push_str(token);
            }
            history.push_str(&self.operand_token);
            history
        }
        else {
            self.temp_history.clone()
        }        
    }

    pub fn perform_exp_input(&mut self, input: String) -> Result<Option<String>, &str> {
        if input.is_empty() {
            return Err("Empty input");
        }
        self.temp_history.clear();

        let immediate_result: Result<Option<String>, &str>;

        loop {
            if input.len() == 1 {
                let c: char = input.chars().next().unwrap();
                if c.is_ascii_digit() || c == '.' {
                    immediate_result = self.expression_operand_input(&c);
                    break;
                }
            }
            let constant = self.constants_map.get(&input);
            match constant {
                Some(value) => {
                    immediate_result = self.expression_constant_input(&value.clone());
                }
                None => {
                    immediate_result = self.expression_op_input(&input);
                }
            }
            break;
        }
        
        immediate_result
    }

    pub fn perform_feature(&mut self, feature: &Feature) -> Result<Option<String>, &str> {
        match feature {
            Feature::CE => {
                self.reset()
            },
            Feature::C => Ok(None),
            Feature::MS => Ok(None),
            Feature::MR => Ok(None),
            Feature::Eval => {
                self.eval()
            },
            Feature::DEL => Ok(None),
        }
    }

    fn eval(&mut self) -> Result<Option<String>, &str> {
        let mut temp_token_updated = false;
        if !self.operand_token.is_empty() {
            let _ = self.evaluator.put_token(&self.operand_token);
            self.input_tokens.push(self.operand_token.clone());
            self.operand_token.clear();
            temp_token_updated = true;
        }
        let res = self.evaluator.evaluate();
        match res {
            Some(v) => {
                // store the final result so that it can be used as the begin of next expression
                self.last_result = v.to_string();
                // reset the evaluator after evaluation
                self.evaluator = Evaluator::new();

                self.temp_history = self.build_history() + " =";
                self.operand_token.clear();
                self.input_tokens.clear();                

                // return the result in String
                Ok(Some(self.last_result.clone()))
            },
            None => {
                // reset the evaluator due to it may damaged by evaluation
                self.evaluator = Evaluator::new();

                // recover evaluator to state before evaluation
                if temp_token_updated {
                    self.operand_token = self.input_tokens.pop().unwrap();
                }
                for token in &self.input_tokens  {
                    let _ = self.evaluator.put_token(token);
                }
                // return none like nothing happened
                Ok(None)
            }
        }
    }

    fn reset(&mut self) -> Result<Option<String>, &str> {
        self.last_result = "0".to_string();
        self.operand_token.clear();
        self.input_tokens.clear();
        self.evaluator = Evaluator::new();
        self.temp_history.clear();

        Ok(Some(self.last_result.clone()))
    }

}