#![allow(unused)]
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Int(i128),
    Float(f64),
    Bool(bool),
    String(String),
    Arithmetic(String),  // For operations like +, -, *, /
    List(Vec<Token>),
    Symbol(String),
    Block(Vec<Token>),
    LogicalOp(String),
    ListOp(String),
}



impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int(value) => write!(f, "{}", value),
            Token::Float(value) => write!(f, "{:?}", value),
            Token::Bool(value) => write!(f, "{}", (if *value {"True"} else {"False"})),
            Token::String(value) => write!(f, "\"{}\"", value),
            Token::Arithmetic(value) => write!(f, "{}", value),
            Token::Symbol(value) => write!(f, "{}", value),
            Token::List(vec) => write!(f, "[{}]", vec.iter().map(|token| token.to_string()).collect::<Vec<_>>().join(",")),
            Token::Block(vec) =>  write!(f, "{{ {} }}", vec.iter().map(|token| token.to_string()).collect::<Vec<_>>().join(" ")),
            Token::LogicalOp(value) => write!(f, "{}", value),
            Token::ListOp(value) => write!(f, "{}", value),
        }
    }
}