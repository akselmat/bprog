#![allow(unused)]

use std::io::empty;
use crate::stack::Stack;
use crate::token::{Token};
use crate::errors::{ProgramError};
use crate::token::Token::Arithmetic;
use std::collections::HashMap;
use std::f32::consts::E;


pub fn interpret(tokens: Vec<Token>) -> Result<Vec<Token>, ProgramError> {
    let mut stack = Stack::new();
    let mut symbols: HashMap<String, Token> = HashMap::new();

    for token in tokens {
        match token.clone() {
            Token::Int(_) | Token::Float(_) | Token::Bool(_) |
            Token::String(_) | Token::Block(_) => {
            // Token::String(_) | Token::List(_) | Token::Block(_) => {
                stack.push(token);
            },
            // Token::List(ref items) => {
            //     let evaluated_list = construct_list(items, &symbols)?;
            //     stack.push(evaluated_list);
            // },
            Token::List(list_tokens) => {
                for l_token in list_tokens {
                    if let Token::Symbol(tok) = l_token {
                        execute_symbol(&tok, &mut symbols, &mut stack)?;
                    } else {
                        stack.push(token.clone());
                    }
                }
                // let evaluated_list = construct_list(items, &symbols)?;
                // stack.push(evaluated_list);
            },
            Token::Symbol(symbol) => {
                handle_symbol(token, &symbol, &mut symbols, &mut stack)?;
            },
            Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => {
                execute_operation(&op, &mut stack)?;
            },
            _ => return Err(ProgramError::UnsupportedType),
        }
    }

    if stack.elements.is_empty() {
        Err(ProgramError::StackEmpty)
    } else {
        Ok(stack.elements)  // Return all remaining elements as a vector
    }
}


fn construct_list(tokens: &[Token], symbols: &HashMap<String, Token>) -> Result<Token, ProgramError> {
    let mut list_items = Vec::new();
    for token in tokens {
        match token {
            Token::List(inner_tokens) => {
                // Recursively evaluate inner lists
                let evaluated_list = construct_list(inner_tokens, symbols)?;
                list_items.push(evaluated_list);
            },
            Token::Symbol(sym) if symbols.contains_key(sym) => {
                // Directly substitute the symbol with its corresponding value
                list_items.push(symbols[sym].clone());
            },
            _ => list_items.push(token.clone()), // For other tokens, add them as they are
        }
    }
    Ok(Token::List(list_items))
}


fn handle_symbol(token: Token, symbol: &str, symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    if symbol == ":=" {
        handle_assignment(symbols, stack)?;
    } else {
        if let Err(_) = execute_symbol(symbol, symbols, stack){
            stack.push(token);
        }
    }
    Ok(())
}

// Handling the execution of a symbol
pub fn execute_symbol(symbol: &str, symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    if let Some(value) = symbols.get(symbol) {
        stack.push(value.clone());
    } else {
        return Err(ProgramError::UnknownSymbol);
    }
    Ok(())
}

fn handle_assignment(symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    if stack.elements.len() < 2 {
        return Err(ProgramError::NotEnoughElements);
    }
    let right = stack.pop()?;
    let left = stack.pop()?;
    // println!(" left: {:?}", left.clone());
    // println!(" rigth: {:?}", right.clone());

    match (left, right.clone()) {
        (Token::Symbol(a), Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) | Token::List(_) | Token::Block(_) ) => {
            symbols.insert(a, right.clone());
            // println!(" symbols.insert(a, right.clone()); ");
        },
        _ => return Err(ProgramError::ExpectedEnumerable),
    }

    Ok(())
}

    // if left == Token::Symbol("".to_string()) && right != Token::Symbol("".to_string()) {
    //     symbols.insert(left.to_string(), right);
    //     println!(" symbols.insert(left.to_string(), right); ");
    //     Ok(())
    // } else {
    //     return Err(ProgramError::UnsupportedType);
    // }







pub fn execute_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    match op {
        // |"cons"|"append"|"each"|"map"
        "+"|"-"|"*"|"/"|"div"|"&&"|"||"|">"|"<"|"=="|"cons"|"append" => binary_op(op, stack),
        "not"|"head"|"tail"|"empty"|"length"|"exec" => unary_op(op, stack),
        "swap"|"dup"|"pop" => stack_op(op, stack),
        // ":=" => handle_assignment(symbols, stack, symbol),
        _ => Err(ProgramError::UnknownOperation),
    }
}


// stack operations
fn stack_op(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    match op {
        "dup" => stack.dup()?,
        "swap" => stack.swap()?,
        "pop" => { stack.pop()?;},
        _ => return Err(ProgramError::UnknownOperation),
    };
    Ok(())
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
        "cons" => cons(left, right)?,
        "append" => append(left, right)?,
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
fn not(right: Token) -> Result<Token, ProgramError> {
    match right {
        Token::Int(a)  => Ok(Token::Int(-(a))),
        Token::Float(a) => Ok(Token::Float(-(a))),
        Token::Bool(a) => Ok(Token::Bool((!a))),
        _ => Err(ProgramError::ExpectedBoolOrNumber),
    }
}
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
    println!("unary_op:");
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
        // "exec" => exec(right)?,
        // "pop" => stack.pop()?,
        // "dup" => stack.dup()?,
        _ => Err(ProgramError::UnsupportedType)?
    };

    stack.push(result);
    Ok(())
}
fn exec(right: Token) -> Result<Token, ProgramError> {
    match right {

        Token::Int(a)  => Ok(Token::Int(-(a))),
        Token::Float(a) => Ok(Token::Float(-(a))),
        Token::Bool(a) => Ok(Token::Bool((!a))),
        _ => Err(ProgramError::ExpectedBoolOrNumber),

    }
}



// List operations
fn head(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) => Ok(Token::String(a[0..1].to_owned())),
        Token::List(a) => Ok(a.first().ok_or(ProgramError::StackEmpty)?.clone()),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn tail(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) =>  Ok(Token::String(a[1..].to_owned())),
        Token::List(a) => Ok(Token::List(a[1..].to_owned())),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn emptyy(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) =>  Ok(Token::Bool(a.is_empty())),
        Token::List(a) => Ok(Token::Bool(a.is_empty())),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}
fn length(right: Token) -> Result<Token, ProgramError> {
    match (right) {
        Token::String(a) => Ok(Token::Int(a.len() as i128)),
        Token::List(a) =>Ok(Token::Int(a.len() as i128)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}

fn cons(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::Int(a), Token::List(b)) => Ok(Token::List([vec![Token::Int(a)], b].concat())),
        (Token::Float(a), Token::List(b)) => Ok(Token::List([vec![Token::Float(a)], b].concat())),
        (Token::Bool(a), Token::List(b)) => Ok(Token::List([vec![Token::Bool(a)], b].concat())),
        (Token::String(a), Token::List(b)) => Ok(Token::List([vec![Token::String(a)], b].concat())),
        (Token::List(a), Token::List(b)) => Ok(Token::List([vec![Token::List(a)], b].concat())),
        _ => Err(ProgramError::ExpectedList),
    }
}
fn append(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (Token::List(mut a), Token::List(b)) => {
            a.extend(b);  // Extend the first vector with the second
            Ok(Token::List(a))  // Return the modified first list as a result
        },
        _ => Err(ProgramError::ExpectedList),
    }
}










// // stack operations
// fn stack_op(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
//     match op {
//         "swap" => stack.swap(),
//         "pop" => {
//             stack.pop()?;  // Pop and discard the top element
//             Ok(())
//         },
//         _ => unreachable!(),
//     }
// }

// pub fn interpret(tokens: Vec<Token>) -> Result<Token, ProgramError> {
//     let mut stack = Stack::new();
//
//     for token in tokens {
//         match token {
//             Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) | Token::List(_) => {
//                 stack.push(token);
//             },
//             Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => execute_operation(&op, &mut stack)?,
//             _=>{
//                 Err(ProgramError::UnsupportedType)?
//             }
//         }
//     }
//
//     // Check that there is exactly one value left on the stack
//     if stack.elements.len() == 1 {
//         Ok(stack.pop()?)
//     } else if stack.elements.is_empty() {
//         Err(ProgramError::StackEmpty)
//     } else {
//         println!("sdfsdfs");
//         Err(ProgramError::ProgramFinishedWithMultipleValues)
//     }
// }

