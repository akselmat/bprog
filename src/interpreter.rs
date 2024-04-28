#![allow(unused)]
use crate::stack::Stack;
use crate::token::{Token};


use crate::errors::{ProgramError};

pub fn interpret(tokens: Vec<Token>) -> Result<Token, ProgramError> {
    let mut stack = Stack::new();

    for token in tokens {
        match token {
            Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) | Token::Block(_) => {
                stack.push(token);
            },
            Token::Arithmetic(op) | Token::LogicalOperations(op) | Token::ListOperations(op) => execute_operation(&op, &mut stack)?,
            // Token::Block(_) => { // Handle block execution, if necessary, for this token type
            // },
            // Other token types and their specific handling can be added here
            _=>{
                Err(ProgramError::UnsupportedType)?
            }
        }
    }

    // Check that there is exactly one value left on the stack
    if stack.elements.len() == 1 {
        Ok(stack.pop()?)
    } else if stack.elements.is_empty() {
        Err(ProgramError::StackEmpty)
    } else {
        Err(ProgramError::ProgramFinishedWithMultipleValues)
    }
}

pub fn execute_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    match op {
        "+" | "-" | "*" | "/"|"div"|"&&"| "||" => binary_op(op, stack),
        "not"|"length" => unary_op(op, stack),
        _ => Err(ProgramError::UnknownOperation),
    }
}



// binary_operation
fn binary_op(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    if stack.elements.len() < 2 {
        return Err(ProgramError::NotEnoughElements);
    }
    let right = stack.pop()?;
    let left = stack.pop()?;

    let result = match op {
        "+" => add(left, right)?,
        "-" => sub(left, right)?,
        "*" => mul(left, right)?,
        "/" => fdiv(left, right)?,
        "div" => div(left, right)?,
        "&&" => and(left, right)?,
        "||" => or(left, right)?,
        _ => unreachable!(), // Since we check op in execute_operation, this should never happen
    };

    stack.push(result);
    Ok(())
}

// simple arithmetic && arithmetic with type coercion
fn add(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a + b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 + b)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a + b as f64)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a + b)),

        // String trenger ikke
        // (Token::String(a), Token::String(b)) => Ok(Token::String(a + &*b)),

        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn sub(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a - b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 - b)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a - b as f64)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a - b)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn mul(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a * b)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a * b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 * b)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a * b as f64)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn fdiv(left: Token, right: Token) -> Result<Token, ProgramError> {
    if let Token::Int(0) | Token::Float(0.0) = right { // check for division by zero
        return Err(ProgramError::DivisionByZero);
    }
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Float((a / b) as f64)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a / b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 / b)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a / b as f64)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn div(left: Token, right: Token) -> Result<Token, ProgramError> {
    if let Token::Int(0) | Token::Float(0.0) = right { // check for division by zero
        return Err(ProgramError::DivisionByZero);
    }
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a / b)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a / b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 / b)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a / b as f64)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}


// bool operations
fn and(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Bool(true), Token::Bool(true)) => Ok(Token::Bool(true)),
        (Token::Bool(false), Token::Bool(true)) => Ok(Token::Bool(false)),
        (Token::Bool(true), Token::Bool(false)) => Ok(Token::Bool(false)),
        (Token::Bool(false), Token::Bool(false)) => Ok(Token::Bool(false)),
        _ => Err(ProgramError::ExpectedBool),
    }
}
fn or(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Bool(true), Token::Bool(true)) => Ok(Token::Bool(true)),
        (Token::Bool(false), Token::Bool(true)) => Ok(Token::Bool(true)),
        (Token::Bool(true), Token::Bool(false)) => Ok(Token::Bool(true)),
        (Token::Bool(false), Token::Bool(false)) => Ok(Token::Bool(false)),
        _ => Err(ProgramError::ExpectedBool),
    }
}


// functinos
fn length(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) => Ok(Token::Int(a.len() as i64 - 2)),
        Token::Block(a) =>Ok(Token::Int(a.len() as i64)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}


// unary_operations:
fn unary_op(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    if stack.elements.is_empty() {
        return Err(ProgramError::StackEmpty);
    }
    let right = stack.pop()?;

    let result = match op {
        "not" => not(right)?,
        "length" => length(right)?,
        _ => unreachable!(), // Since we check op in execute_operation, this should never happen
    };

    stack.push(result);
    Ok(())
}

fn not(right: Token) -> Result<Token, ProgramError> {
    match right {
        Token::Int(a)  => Ok(Token::Int(-(a))),
        Token::Float(a) => Ok(Token::Float(-(a))),
        Token::Bool(a) => Ok(Token::Bool((!a))),
        _ => Err(ProgramError::ExpectedBoolOrNumber),
    }
}
