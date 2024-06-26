#![allow(unused)]
use crate::stack::Stack;
use crate::token::{Token};
use crate::errors::{ProgramError};
use std::collections::HashMap;
use crate::parser::{Parser};
use std::io::{self, Read, Write};



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

    // Set new tokens for the interpreter
    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    pub fn interpret(&mut self) -> Result<Stack, ProgramError> {
        process_tokens_recursively(&self.tokens, &mut self.symbols, &mut self.stack);
        // After processing all tokens, check if the stack is empty
        if self.stack.elements.is_empty() {
            Err(ProgramError::StackEmpty)
        } else {
            Ok(self.stack.clone()) // Return a copy of the stack's elements
        }
    }

    pub fn run_normal_mode(&mut self) {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read input");
        // let input = " [ { + } { 10 + } { 20 10 + } ]  ";
        // let input = " \" 12 \" parseInteger   ";
        match Parser::new(&input.trim()).parse_tokens() {
            Ok(tokens) => {
                self.set_tokens(tokens);
                match self.interpret() {
                    Ok(stack) => {
                        // println!("{:?}", stack);
                        if stack.elements.len() > 1 {
                            println!("Error: {:?}", ProgramError::ProgramFinishedWithMultipleValues);
                        } else {
                            println!("{}", stack);
                        }
                    },
                    Err(e) => println!("Error: {:?}", e),
                }
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }

    // Run the REPL mode
    pub fn run_repl_mode(&mut self) {
        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            print!("bprog> ");
            io::stdout().flush().unwrap();

            input.clear(); // Clear the previous input
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    if input.trim() == "exit" { break; } // Exit command to quit REPL
                    match Parser::new(&input.trim()).parse_tokens() {
                        Ok(tokens) => {
                            self.set_tokens(tokens);
                            match self.interpret() {
                                Ok(stack) => {
                                    println!("{}", stack); // Using the Display trait of Stack
                                    if stack.elements.len() > 1 {
                                        println!("--"); // Indicate multiple values on stack
                                    }
                                },
                                Err(e) => println!("Error: {:?}", e),
                            }
                        },
                        Err(e) => println!("Error: {:?}", e),
                    }
                },
                Err(e) => println!("Error reading line: {}", e),
            }
        }
    }

}


pub fn process_tokens_recursively<'a>(tokens: &[Token], symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    for token in tokens {
        match token {
            Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) => stack.push(token.clone()),
            Token::Symbol(sym) => handle_symbol(&sym, symbols, stack)?,
            Token::Block(inner_tokens) => {
                // Push the whole block onto the stack without executing it
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
                process_tokens_recursively(inner_tokens, symbols, &mut list_stack)?;
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
        (Token::List(elements), Token::Block(ops)) => {
            let mut new_list = vec![];
            // Apply the block to each element in the list
            for element in elements {
                stack.push(element.clone()); // Push the element onto the stack
                process_tokens_recursively(&ops, &mut HashMap::new(), stack)?; // Execute the block
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
    let block_token = stack.pop()?;
    // println!("block_token {:?}", block_token);
    if let Token::Block(inner_tokens) = block_token {
        // Recursively interpret the block's tokens
        // println!("interpretor(&inner_tokens, symbols, stack)?!");
        process_tokens_recursively(&inner_tokens, symbols, stack)?;
    } else {
        return Err(ProgramError::ExpectedQuotation);
    }
    Ok(())
}


pub fn execute_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    match op {
        "+"|"-"|"*"|"/"|"div"|"&&"|"||"|">"|"<"|"=="|"cons"|"append"|"map" => binary_op(op, stack),
        "not"|"head"|"tail"|"empty"|"length"|"parseInteger"|"parseFloat" => unary_op(op, stack),
        "swap"|"dup"|"pop" => stack_op(op, stack),
        // ":=" => handle_assignment(symbols, stack, symbol),
        _ => Err(ProgramError::UnknownOperation),
    }
}

fn handle_symbol(symbol: &str, symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    match symbol {
        "exec" => execute_block(symbols, stack)?,
        "fun" => define_func(symbols, stack)?,
        "map" => {
            symbols.insert("map".to_string(), Token::Symbol("".to_string()));
        },
        ":=" => handle_assignment(symbols, stack)?,
        _ => {
            execute_symbol(symbol, symbols, stack)?
        }
    }
    Ok(())
}


fn define_func(symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    if stack.elements.len() < 2 {
        return Err(ProgramError::NotEnoughElements);
    }
    let right = stack.pop()?;
    let left = stack.pop()?;

    match (left, &right) {
        (Token::Symbol(a), Token::Block(b)) => {
            symbols.insert(a, right);  // Assign any function token to the symbol
        },
        _ => return Err(ProgramError::ExpectedVariable),
    }
    Ok(())
}
fn execute_symbol(symbol: &str, symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    if let Some(token_value) = symbols.get(symbol) { // if key exist then get the value of that symbol
        if let Token::Block(inner_tokens) = token_value { // if a function
            // println!("is a block value!! {:?}:", inner_tokens.clone());
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
        _ => return Err(ProgramError::ExpectedVariable),
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
    // println!("unary_op:");
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
        "parseInteger" => parse_integer(right)?,  // Add this case
        "parseFloat" => parse_float(right)?,  // Add this case
        _ => Err(ProgramError::UnsupportedType)?
    };

    stack.push(result);
    Ok(())
}
fn parse_integer(token: Token) -> Result<Token, ProgramError> {
    match token {
        Token::String(a) => {
            if let Ok(value) = a.parse::<i128>(){
                return Ok(Token::Int(value))
            } else {
              return Err(ProgramError::NumberConversionError)
            }
        }
        _=> Err(ProgramError::ExpectedVariable)
    }
}
fn parse_float(token: Token) -> Result<Token, ProgramError> {
    match token {
        Token::String(a) => {
            if let Ok(value) = a.parse::<f64>(){
                return Ok(Token::Float(value))
            } else {
                return Err(ProgramError::NumberConversionError)
            }
        }
        _=> Err(ProgramError::ExpectedVariable)
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
        "cons" => cons(left, right)?,
        "append" => append(left, right)?,
        // "map" => map(left, right)?,
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


// list operations
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