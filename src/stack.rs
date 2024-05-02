#![allow(unused)]

use std::fmt;
use crate::errors::ProgramError;
use crate::token::Token;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Stack {
    pub elements: Vec<Token>,
}
impl Stack {
    pub fn new() -> Self {
        Stack { elements: Vec::new() }
    }
    pub fn get_elements(&mut self) -> Result<Vec<Token>, ProgramError> {
        Ok(self.elements.clone())
    }

    pub fn push(&mut self, token: Token) {
        self.elements.push(token);
    }
    pub fn pop(&mut self) -> Result<Token, ProgramError> {
        self.elements.pop().ok_or(ProgramError::StackEmpty)
    }

    pub fn top(&self) -> Result<&Token, ProgramError> {
        self.elements.last().ok_or(ProgramError::StackEmpty)
    }

    pub fn dup(&mut self) -> Result<(), ProgramError> {
        let top_element =  self.top()?;
        self.push(top_element.clone());
        Ok(())
    }

    pub fn swap(&mut self) -> Result<(), ProgramError> {
        let right = self.pop()?;
        let left = self.pop()?;
        self.push(right);
        self.push(left);
        Ok(())
    }

}

// Implement the Display trait for the State struct.
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.elements.iter().rev().map(|c| c.to_string()).collect::<Vec<String>>().join(","))
    }
}



