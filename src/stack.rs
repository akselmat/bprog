#![allow(unused)]
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
        // jeg tror dette er forkortelese self.elements.pop().ok_or(ProgramError::StackEmpty)
        match self.elements.pop(){
            Some(token) => Ok(token),
            None => Err(ProgramError::StackEmpty),
        }
    }
    pub fn top(&self) -> Result<&Token, ProgramError> {
    // pub fn top(&self) -> Option<&Token> {
        // self.elements.last()
        match self.elements.last(){
            Some(token) => Ok(token),
            None => Err(ProgramError::StackEmpty),
        }
    }
}