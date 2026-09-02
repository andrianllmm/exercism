use std::{collections::HashMap};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self { stack: Vec::new(), words: HashMap::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let tokens = Forth::tokenize(input);
        let mut pos = 0;
        while pos < tokens.len() {
            self.eval_token(&tokens, &mut pos)?;
        }

        Ok(())
    }

    pub fn eval_token(&mut self, tokens: &[String], pos: &mut usize) -> Result {
        let token = &tokens[*pos];

        let num = token.parse::<i32>().ok();
        if num != None {
            self.stack.push(num.unwrap());
            *pos += 1;
            return Ok(());
        }

        if token == ":"  {
            self.define_word(tokens, pos)?;
            *pos += 1;
            return Ok(())
        }

        if self.words.contains_key(token.to_lowercase().as_str()) {
            self.execute_word(token)?;
            *pos += 1;
            return Ok(())
        }

        if "+-*/".contains(&token.as_str()) {
            self.execute_arithmetic(&token)?;
            *pos += 1;
            return Ok(());
        }

        if ["drop", "dup", "swap", "over"].contains(&token.to_lowercase().as_str()) {
            self.execute_stack_op(token)?;
            *pos += 1;
            return Ok(());
        }

        Err(Error::UnknownWord)
    }

    pub fn execute_arithmetic(&mut self, op: &str) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }

        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        let res = match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => {
                if right == 0 {
                    return Err(Error::DivisionByZero);
                }
                left / right
            },
            _ => 0,
        };

        self.stack.push(res);

        Ok(())
    }

    pub fn execute_stack_op(&mut self, op: &str) -> Result {
        match op.to_lowercase().as_str() {
            "drop" => {
                if self.stack.len() == 0 {
                    return Err(Error::StackUnderflow);
                }
                self.stack.pop();
            }
            "dup" => {
                if self.stack.len() == 0 {
                    return Err(Error::StackUnderflow);
                }
                if let Some(num) = self.stack.last() {
                    self.stack.push(*num);
                }
            },
            "swap" => {
                if self.stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                }
                let last = self.stack.pop().unwrap();
                let last2 = self.stack.pop().unwrap();
                self.stack.push(last);
                self.stack.push(last2);
            },
            "over" => {
                if self.stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                }
                let last2 = self.stack[self.stack.len() - 2];
                self.stack.push(last2);
            },
            _ => {}
        }

        Ok(())
    }

    pub fn define_word(&mut self, tokens: &[String], pos: &mut usize) -> Result {
        *pos += 1;

        if *pos >= tokens.len() {
            return Err(Error::InvalidWord);
        }

        let name = tokens[*pos].to_lowercase().to_string();

        if name.parse::<i32>().is_ok() {
            return Err(Error::InvalidWord);
        }

        *pos += 1;

        let mut commands = Vec::new();

        while *pos < tokens.len() && tokens[*pos] != ";" {
            let command = &tokens[*pos];

            if let Some(existing) = self.words.get(&command.to_lowercase()) {
                commands.extend(existing.clone());
            } else {
                commands.push(command.clone());
            }

            *pos += 1;
        }

        if *pos >= tokens.len() {
            return Err(Error::InvalidWord);
        }

        self.words.insert(name, commands);

        Ok(())
    }

    pub fn execute_word(&mut self, name: &str) -> Result {
        let commands = self
            .words
            .get(&name.to_lowercase())
            .cloned()
            .ok_or(Error::UnknownWord)?;

        let mut pos = 0;
        while pos < commands.len() {
            self.eval_token(&commands, &mut pos)?;
        }

        Ok(())
    }

    pub fn tokenize(input: &str) -> Vec<String> {
        input
            .split_whitespace()
            .map(str::to_string)
            .collect()
    }
}
