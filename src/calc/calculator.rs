pub struct Calculator {
    pub count: u32,
}


impl Calculator {
    pub fn new() -> Self {
        Self {
            count: 0
        }
    }

    pub fn push_input(&mut self, input: char) -> (Option<String>, Option<String>) {
        self.count += 1;
        let history = self.count.to_string();
        (Some(history), Some(input.to_string()))
    }
}