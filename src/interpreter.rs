#![allow(unused)]

use crate::stack::Stack;
use crate::token::{Token};
use crate::errors::{ParserError, ProgramError};
use crate::token::Token::{Arithmetic, Block};
use std::collections::HashMap;
use std::f32::consts::E;
use crate::parser::{Parser, split_into_tokens};
use crate::t;

// Token::Symbol(sym) => {
//     handle_symbol(&sym, symbols, stack)?;
// },
// Token::Symbol(sym) if sym == "exec" => {
//     // Execute the top block if 'exec' is encountered
//     execute_block(symbols, stack)?;
// },
// Token::Block(inner_tokens) => {
//     // Optionally create a new stack or use the existing one
//     // depending on whether you want isolated or shared stack spaces
//     // interpretor(inner_tokens, symbols, stack)?;
// },


#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    stack: Stack,
    symbols: HashMap<String, Token>,
    tokens: Vec<Token>,
}

impl Interpreter {
    // Constructor that initializes an interpreter with a list of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            stack: Stack::new(),
            symbols: HashMap::new(),
            tokens,
        }
    }

    // Method to execute the interpreter
    pub fn interpret(&mut self) -> Result<Vec<Token>, ProgramError> {
        interpretor(&self.tokens, &mut self.symbols, &mut self.stack);
        // After processing all tokens, check if the stack is empty
        if self.stack.elements.is_empty() {
            Err(ProgramError::StackEmpty)
        } else {
            Ok(self.stack.elements.clone()) // Return a copy of the stack's elements
        }
    }
}

// prøver noen nytt!!
pub fn interpretor<'a>(tokens: &[Token], symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    for token in tokens {
        match token {
            Token::Int(value) => {
                stack.push(Token::Int(*value));
            },
            Token::Float(value) => {
                stack.push(Token::Float(*value));
            },
            Token::Bool(value) => {
                stack.push(Token::Bool(*value));
            },
            Token::String(value) => {
                stack.push(Token::String(value.clone()));
            },
            Token::Symbol(sym) => {
                handle_symbol(&sym, symbols, stack)?;
            },
            Token::Block(inner_tokens) => {
                // Push the whole block onto the stack without executing it
                println!("if BLOCKK");
                if let Some(_) = symbols.get("map") {
                    stack.push(Token::Block(inner_tokens.clone())); // push the block into the stack
                    execute_map(stack)?;
                } else {
                    stack.push(Token::Block(inner_tokens.clone()));
                }
            },
            Token::List(inner_tokens) => {
                // For a list, you might want to push the entire evaluated list back onto the stack
                let mut list_stack = Stack::new();
                interpretor(inner_tokens, symbols, &mut list_stack)?;
                stack.push(Token::List(list_stack.elements));
            },
            Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => {
                execute_operation(&op, stack)?;
            },
            _ => return Err(ProgramError::UnsupportedType),
        }
    }
    Ok(())
}

fn execute_map(stack: &mut Stack) -> Result<(), ProgramError> {
    // Pop the quotation and the list from the stack
    let quotation = stack.pop()?;
    let list_token = stack.pop()?;

    match (list_token, quotation) {
        // (Token::List(elements), Token::Block(ops) | Token::Symbol(op)) => {
        (Token::List(elements), Token::Block(ops)) => {
            let mut new_list = vec![];
            // Apply the block to each element in the list
            for element in elements {
                stack.push(element.clone()); // Push the element onto the stack
                interpretor(&ops, &mut HashMap::new(), stack)?; // Execute the block
                if let Ok(result) = stack.pop() {
                    new_list.push(result); // Collect the result
                }
            }
            stack.push(Token::List(new_list)); // Push the new list onto the stack
            Ok(())
        },
        _ => Err(ProgramError::ExpectedListAndQuotation),
    }
}




// Function to execute a block
fn execute_block(symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    println!("FN execute_block ------------------------");  //--
    println!("current: stack: {:?}", stack.elements.clone()); // current stack
    let block_token = stack.pop()?;
    println!("block_token {:?}", block_token);
    if let Token::Block(inner_tokens) = block_token {
        // Recursively interpret the block's tokens
        println!("interpretor(&inner_tokens, symbols, stack)?!");
        interpretor(&inner_tokens, symbols, stack)?;
    } else {
        return Err(ProgramError::ExpectedQuotation);
    }
    Ok(())
}


pub fn execute_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    match op {
        // |"cons"|"append"|"each"|"map"
        "+"|"-"|"*"|"/"|"div"|"&&"|"||"|">"|"<"|"=="|"cons"|"append"|"map" => binary_op(op, stack),
        "not"|"head"|"tail"|"empty"|"length" => unary_op(op, stack),
        "swap"|"dup"|"pop" => stack_op(op, stack),
        // ":=" => handle_assignment(symbols, stack, symbol),
        _ => Err(ProgramError::UnknownOperation),
    }
}

fn handle_symbol(symbol: &str, symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    println!("FN handle_symbol-------------------------");
    match symbol {
        "exec" => execute_block(symbols, stack)?,
        "fun" => define_func(symbols, stack)?,
        "map" => {
            symbols.insert("map".to_string(), Token::Symbol("".to_string()));
        println!("insert into the map: ");
        },
        ":=" => handle_assignment(symbols, stack)?,
        _ => {
            execute_symbol(symbol, symbols, stack)?
        }
    }
    Ok(())
}



fn define_func(symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    println!("FN define_func!!!!!!!!!!!!");
    if stack.elements.len() < 2 {
        return Err(ProgramError::NotEnoughElements);
    }
    println!("define func: stack elements:{:?}", stack.elements.clone());
    let right = stack.pop()?;
    let left = stack.pop()?;
    println!("define_func left: {:?}", left.clone());
    println!("define_func right: {:?}", right.clone());

    match (left, &right) {
        (Token::Symbol(a), Token::Block(b)) => {
            println!("symbols.insert: {:?}", right.clone());
            symbols.insert(a, right);  // Assign any function token to the symbol
        },
        _ => return Err(ProgramError::ExpectedVariable("Expected a symbol for variable assignment.".to_string())),
    }
    Ok(())
}
fn execute_symbol(symbol: &str, symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    println!("FN execute_symbol!!!!!!!!!!!!!!!!!");
    println!("stack elements: {:?}:", stack.elements.clone());
    if let Some(token_value) = symbols.get(symbol) { // if key exist then get the value of that symbol
        if let Token::Block(inner_tokens) = token_value { // if a function
            println!("is a block value!! {:?}:", inner_tokens.clone());
            stack.push(token_value.clone());               // push tokens to stack
            execute_block(symbols, stack)?
        } else {
            stack.push(token_value.clone());               // if a variable then push to stack
        }
    } else {                                                // if the symbol dosen't exist
        stack.push(Token::Symbol(symbol.to_string()));      // push it to the stack
    }
    Ok(())
}


// Handling the execution of a symbol
// fn execute_symbol(symbol: &str, symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
//     if let Some(value) = symbols.get(symbol) { // if key exist then get the value of that symbol
//         stack.push(value.clone());             // and push it into stack
//     } else {
//         return Err(ProgramError::UnknownSymbol);
//     }
//     Ok(())
// }

fn handle_assignment(symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    if stack.elements.len() < 2 {
        return Err(ProgramError::NotEnoughElements);
    }
    let right = stack.pop()?;
    let left = stack.pop()?;

    match (left, &right) {
        (Token::Symbol(a), Token::Symbol(_)) => {
            return Err(ProgramError::UnsupportedType);  // Prevent symbol to symbol assignment
        },
        (Token::Symbol(a), _) => {
            symbols.insert(a, right);  // Assign any non-symbol token to the symbol
        },
        _ => return Err(ProgramError::ExpectedVariable("Expected a symbol for variable assignment.".to_string())),
    }
    Ok(())
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
        _ => Err(ProgramError::UnsupportedType)?
    };

    stack.push(result);
    Ok(())
}

// binary_operation
fn binary_op(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    println!("FN binary_op--------------:");
    println!("stack elements: {:?}", stack.elements.clone());
    if stack.elements.len() < 2 {
        return Err(ProgramError::NotEnoughElements);
    }
    let right = stack.pop()?;
    println!("rigth: {:?}",right.clone());
    let left = stack.pop()?;
    println!("left: {:?}", left.clone());

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
        // "map" => map(left, right)?,
        _ => unreachable!(), // Since we check op in execute_operation, this should never happen
    };

    stack.push(result);
    Ok(())
}
// // List operations
// fn map(left: Token, right: Token) -> Result<Token, ProgramError> {
//     println!("rigth: {:?}",right.clone());
//     println!("left: {:?}", left.clone());
//
//
//
//     // Ok(Token::Symbol(right))
//     // Err()
//
// }




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
    match right {
        Token::Int(0) => return Err(ProgramError::DivisionByZero),
        Token::Float(b) if b == 0.0 => return Err(ProgramError::DivisionByZero),
        _ => {},
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
    match right {
        Token::Int(0) => return Err(ProgramError::DivisionByZero),
        Token::Float(b) if b == 0.0 => return Err(ProgramError::DivisionByZero),
        _ => {},
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
        Token::Block(a) =>Ok(Token::Int(a.len() as i128)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}

fn cons(left: Token, right: Token) -> Result<Token, ProgramError> {
    match (left, right) {
        (a, Token::List(b)) => Ok(Token::List([vec![a], b].concat())),
        _ => Err(ProgramError::ExpectedList),
    }

    // match (left, right) {
    //     (Token::Int(a), Token::List(b)) => Ok(Token::List([vec![Token::Int(a)], b].concat())),
    //     (Token::Float(a), Token::List(b)) => Ok(Token::List([vec![Token::Float(a)], b].concat())),
    //     (Token::Bool(a), Token::List(b)) => Ok(Token::List([vec![Token::Bool(a)], b].concat())),
    //     (Token::String(a), Token::List(b)) => Ok(Token::List([vec![Token::String(a)], b].concat())),
    //     (Token::List(a), Token::List(b)) => Ok(Token::List([vec![Token::List(a)], b].concat())),
    //     _ => Err(ProgramError::ExpectedList),
    // }


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
