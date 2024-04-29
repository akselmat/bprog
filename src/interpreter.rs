#![allow(unused)]

use std::io::empty;
use crate::stack::Stack;
use crate::token::{Token};
use crate::errors::{ProgramError};


pub fn interpret(tokens: Vec<Token>) -> Result<Token, ProgramError> {
    let mut stack = Stack::new();

    for token in tokens {
        match token {
            Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) | Token::List(_) => {
                stack.push(token);
            },
            Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => execute_operation(&op, &mut stack)?,
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
        // "head"|"tail"|"empty"|"length"|"cons"|"append"|"each"|"map"
        // |"cons"|"append"|"each"|"map"
        "+"|"-"|"*"|"/"|"div"|"&&"|"||"|">"|"<"|"==" => binary_op(op, stack),
        "not"|"head"|"tail"|"empty"|"length" => unary_op(op, stack),
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
        ">" => less_then_left(left, right)?,
        "<" => less_then_right(left, right)?,
        "==" => equal(left, right)?,
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
        (Token::Float(a), Token::Float(b)) => Ok(Token::Int((a / b) as i128 )),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Int(a / b  as i128 )),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Int(a as i128 / b )),
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
fn less_then_left(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Bool(a > b)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Bool(a > b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Bool(a > b as i128)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Bool(a > b as f64)),
        (Token::String(a), Token::Int(b)) => Ok(Token::Bool(a.len() > b as usize)),
        (Token::String(a), Token::Float(b)) => Ok(Token::Bool(a.len() > b as usize)),
        (Token::String(a), Token::String(b)) => Ok(Token::Bool(a.len() > a.len())),

        (Token::Bool(a), Token::Bool(b)) => Ok(Token::Bool(a > a)),
        (Token::List(a), Token::Bool(b)) => Ok(Token::Bool(a.len() > b as usize)),
        (Token::Bool(a), Token::List(b)) => Ok(Token::Bool(a as usize > b.len())),
        (Token::List(a), Token::List(b)) => Ok(Token::Bool(a.len()  > b.len())),
        _ => Err(ProgramError::ExpectedBool),
    }
}
fn less_then_right(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Bool(a < b)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Bool(a < b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Bool(a < b as i128)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Bool(a < b as f64)),
        (Token::String(a), Token::Int(b)) => Ok(Token::Bool(a.len() < b as usize)),
        (Token::String(a), Token::Float(b)) => Ok(Token::Bool(a.len() < b as usize)),
        (Token::String(a), Token::String(b)) => Ok(Token::Bool(a.len() < a.len())),

        (Token::Bool(a), Token::Bool(b)) => Ok(Token::Bool(a < a)),
        (Token::List(a), Token::Bool(b)) => Ok(Token::Bool(a.len() < b as usize)),
        (Token::Bool(a), Token::List(b)) => Ok(Token::Bool((a as usize) < b.len())),
        (Token::List(a), Token::List(b)) => Ok(Token::Bool(a.len() < b.len())),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn equal(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Int(a), Token::Int(b)) => Ok(Token::Bool(a == b)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Bool(a == b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Bool(a == b as i128)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Bool(a == b as f64)),
        (Token::String(a), Token::Int(b)) => Ok(Token::Bool(a.len() == b as usize)),
        (Token::String(a), Token::Float(b)) => Ok(Token::Bool(a.len() == b as usize)),
        (Token::String(a), Token::String(b)) => Ok(Token::Bool(a.len() == a.len())),

        (Token::Bool(a), Token::Bool(b)) => Ok(Token::Bool(a == a)),
        (Token::List(a), Token::Bool(b)) => Ok(Token::Bool(a.len() == b as usize)),
        (Token::Bool(a), Token::List(b)) => Ok(Token::Bool((a as usize) == b.len())),

        (Token::List(a), Token::List(b)) => Ok(Token::Bool(a.len()  == b.len())),
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
        "head" => head(right)?,
        "tail" => tail(right)?,
        "empty" => emptyy(right)?,
        "length" => length(right)?,
        _ => Err(ProgramError::UnsupportedType)?
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

// functinos
fn head(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) => Ok(Token::String(a[0..1].to_owned())),
        Token::List(a) => Ok(a.first().ok_or(ProgramError::StackEmpty)?.clone()),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
// functinos
fn tail(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) =>  Ok(Token::String(a[1..].to_owned())),
        Token::List(a) => Ok(Token::List(a[1..].to_owned())),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
// functinos
fn emptyy(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) =>  Ok(Token::Bool(a.is_empty())),
        Token::List(a) => Ok(Token::Bool(a.is_empty())),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}

// functinos
fn length(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) => Ok(Token::Int(a.len() as i128)),
        Token::List(a) =>Ok(Token::Int(a.len() as i128)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}

