use std::ops::Sub;
use std::ops::Add;
use crate::errors::ProgramError;

#[derive(Debug, Clone, PartialEq, PartialOrd)] // må kanskje endre dette
pub enum Token {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Arithmetic(String),  // For operations like +, -, *, /
    Block(Vec<Token>),   // For blocks or quotations
}



// pub trait ArithmeticOps {
//     fn add(self, other: Self) -> Result<Token, ProgramError>;
//     fn sub(self, other: Self) -> Result<Token, ProgramError>;
//     fn mul(self, other: Self) -> Result<Token, ProgramError>;
//     fn div(self, other: Self) -> Result<Token, ProgramError>;
// }
//
// impl ArithmeticOps for Token {
//     fn add(self, other: Self) -> Result<Token, ProgramError> {
//         match (self, other) {
//             (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a + b)),
//             (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a + b)),
//             (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 + b)),
//             (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a + b as f64)),
//             _ => Err(ProgramError::ExpectedEnumerable),
//         }
//     }
//     // Implement sub, mul, div similarly
//     fn sub(self, other: Self) -> Result<Token, ProgramError> {
//         match (self, other) {
//             (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a - b)),
//             (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a - b)),
//             (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 - b)),
//             (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a - b as f64)),
//             _ => Err(ProgramError::ExpectedEnumerable),
//         }
//     }
//     fn mul(self, other: Self) -> Result<Token, ProgramError> {
//         match (self, other) {
//             (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a * b)),
//             (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a * b)),
//             (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 * b)),
//             (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a * b as f64)),
//             _ => Err(ProgramError::ExpectedEnumerable),
//         }
//     }
//     fn div(self, other: Self) -> Result<Token, ProgramError> {
//         match (self, other) {
//             (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a / b)),
//             (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a / b)),
//             (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 / b)),
//             (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a / b as f64)),
//             _ => Err(ProgramError::ExpectedEnumerable),
//         }
//     }
//
// }



// impl Add for Token {
//     type Output = Result<Self, String>;
//     fn add(self, other: Self) -> Self::Output {
//         match (self, other) {
//             (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a + b)),
//             (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a + b)),
//             (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 + b)),
//             (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a + b as f64)),
//             // Handle other cases or type mismatches
//             _ => Err("Operation not supported for given types".to_string()),
//         }
//     }
// }
//
// impl Sub for Token {
//     type Output = Result<Self, String>;
//     fn sub(self, other: Self) -> Self::Output {
//         match (self, other) {
//             (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a - b)),
//             (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a - b)),
//             (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 - b)),
//             (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a - b as f64)),
//             // Handle other cases or type mismatches
//             _ => Err("Operation not supported for given types".to_string()),
//         }
//     }
// }










// pub enum Token {
//     Integers(i32),
//     Floats(f32),
//     Bools(true||false),
//     Operator(String),
//     Identifier(String),
//     Space,
//     OpenBracket,
//     CloseBracket,
//     OpenBrace,
//     CloseBrace,
//     EndOfLine,
// }