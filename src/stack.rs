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

    // Duplicates the top element on the stack
    pub fn dup(&mut self) -> Result<(), ProgramError> {
        let top_element = self.top()?.clone(); // Clone the top element
        self.push(top_element); // Push it back onto the stack
        Ok(())
    }

    // Swaps the two top elements on the stack
    pub fn swap(&mut self) -> Result<(), ProgramError> {
        let right = self.pop()?;
        let left = self.pop()?;
        self.push(right);
        self.push(left);
        Ok(())
    }
    // pub fn swap(&mut self) -> Result<(), ProgramError> {
    //     if self.elements.len() < 2 {
    //         return Err(ProgramError::StackEmpty);
    //     }
    //     let top_index = self.elements.len() - 1;
    //     let second_index = self.elements.len() - 2;
    //     self.elements.swap(top_index, second_index); // Swap the elements in the vector
    //     Ok(())
    // }

    // Removes the top element from the stack (already implemented as `pop()`)
    // Adding `discard` method to emphasize the operation's purpose in some contexts
    pub fn discard(&mut self) -> Result<(), ProgramError> {
        self.pop().map(|_| ()) // Pop the top element and ignore it
    }

}



