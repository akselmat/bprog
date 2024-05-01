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
        println!("top element:{:?}", self.elements.last().clone());
        let top_element =  self.elements.last().ok_or(ProgramError::StackEmpty)?.clone();
        println!("top element:{:?}", top_element.clone());
        self.push(top_element.clone());
        Ok(())
        // Ok(top_element)
    }

    pub fn swap(&mut self) -> Result<(), ProgramError> {
        let right = self.pop()?;
        let left = self.pop()?;
        self.push(right);
        self.push(left);
        Ok(())
    }

    // Removes the top element from the stack (already implemented as `pop()`)
    // Adding `discard` method to emphasize the operation's purpose in some contexts
    pub fn discard(&mut self) -> Result<(), ProgramError> {
        self.pop().map(|_| ()) // Pop the top element and ignore it
    }

}



