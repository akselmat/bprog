use crate::stack::Stack;
use crate::token::{Token};


use crate::errors::{ProgramError};

pub fn interpret(tokens: Vec<Token>) -> Result<Token, ProgramError> {
    let mut stack = Stack::new();

    for token in tokens {
        match token {
            Token::Int(_) | Token::Float(_) | Token::Boolean(_) | Token::String(_)  => {
                stack.push(token);
            },
            Token::Arithmetic(op) => execute_operation(&op, &mut stack)?,
            Token::Block(_) => { // Handle block execution, if necessary, for this token type
            },
            // Other token types and their specific handling can be added here
            _=>{}

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
        "+" | "-" | "*" | "/" => execute_binary_operation(op, stack),
        "neg" | "not" => execute_unary_operation(op, stack),
        _ => Err(ProgramError::UnknownOperation),
    }
}

fn execute_binary_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
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
        _ => unreachable!(), // Since we check op in execute_operation, this should never happen
    };

    stack.push(result);
    Ok(())
}
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
        (Token::Int(a), Token::Int(b)) => Ok(Token::Int(a / b)),
        (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a / b)),
        (Token::Int(a), Token::Float(b)) => Ok(Token::Float(a as f64 / b)),
        (Token::Float(a), Token::Int(b)) => Ok(Token::Float(a / b as f64)),
        _ => Err(ProgramError::ExpectedEnumerable),
    }
}

fn execute_unary_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
    if stack.elements.is_empty() {
        return Err(ProgramError::StackEmpty);
    }

    let operand = stack.pop()?;

    let result = match op {
        // "neg" => negate_token(operand)?,
        // "not" => not_token(operand)?,
        _ => unreachable!(), // Since we check op in execute_operation, this should never happen
    };

    stack.push(result);
    Ok(())
}









// pub fn interpret(tokens: Vec<Token>) -> Result<Token, ProgramError> {
//     let mut stack = Stack::new();
//
//     for token in tokens {
//         match token {
//             Token::Int(_) | Token::Float(_) | Token::Boolean(_) | Token::String(_) => stack.push(token),
//             Token::Arithmetic(op) => execute_operation(&op, &mut stack)?,
//             _ => return Err(ProgramError::UnexpectedToken),
//         }
//     }
//
//     if stack.elements.len() == 1 {
//         Ok(stack.pop()?)
//     } else {
//         Err(ProgramError::ProgramFinishedWithMultipleValues)
//     }
// }
//
// pub fn execute_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
//     // Ensure there are enough elements for binary operations
//     if stack.elements.len() < 2 {
//         return Err(ProgramError::NotEnoughElements);
//     }
//
//     let right = stack.pop()?;
//     let left = stack.pop()?;
//
//     let result = match (op, left, right) {
//         ("+", Token::Int(a), Token::Int(b)) => Token::Int(a + b),
//         ("-", Token::Int(a), Token::Int(b)) => Token::Int(a - b),
//         ("*", Token::Int(a), Token::Int(b)) => Token::Int(a * b),
//         ("/", Token::Int(a), Token::Int(b)) => {
//             if b == 0 { return Err(ProgramError::DivisionByZero); }
//             Token::Int(a / b)
//         },
//         // Add cases for float and mixed operations
//         _ => return Err(ProgramError::UnsupportedType),
//     };
//
//     stack.push(result);
//     Ok(())
// }



// pub fn interpret(tokens: Vec<Token>) -> Result<(), ProgramError> {
//     let mut stack = Stack::new();
//     if tokens.len() < 3 {
//         return Err(ProgramError::NotEnoughElementss)
//     }
//
//
//     for token in tokens {
//         match token {
//             Token::Int(num) => stack.push(Token::Int(num)),
//             Token::Float(num) => stack.push(Token::Float(num)),
//
//             Token::Arithmetic(op) => execute_operation(&op, &mut stack)?,
//
//             // Handle other tokens appropriately
//             _ => (),
//         }
//     }
//     Ok(())
// }


// pub fn execute_operation(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
//     let right = stack.pop()?;
//     let left = stack.pop()?;
//     let result = match op {
//         "+" => left.add(right),
//         "-" => left.sub(right),
//         "*" => left.mul(right),
//         "/" => left.div(right),
//         _ => Err(ProgramError::UnknownOperation),
//     };
//     stack.push(result?); // push token into the stack
//     println!("stack: {:?}", stack.elements); // print the stack
//     Ok(())
// }











// pub fn execute_operation(op: &str, stack: &mut Stack) {
//     match op {
//         "+" => {
//             if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
//
//                 match left + right {
//                     Ok(result) => stack.push(result),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             } else {
//                 println!("Error: Stack underflow");
//             }
//         },
//         "-" => {
//             if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
//                 match left - right {
//                     Ok(result) => stack.push(result),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             } else {
//                 println!("Error: Stack underflow");
//             }
//         },
//         // Add other operations and handle different token types similarly
//         _ => println!("Unknown operation"),
//     }
// }


// pub fn execute_operation(op: &str, stack: &mut Stack) {
//     match op {
//         "+" => {
//             (Token::Float(a), Token::Float(b)) => Ok(Token::Float(a + b)),
//             if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
//                 if let (Token::Int(a), Token::Int(b)) = (left, right) {
//                     stack.push(Token::Int(a + b));
//                 }
//             } else {
//                 println!("Error: Stack underflow");
//             }
//         },
//         // Add other operations and handle different token types similarly
//         _ => println!("Unknown operation"),
//     }
// }


// pub fn execute_operation(op: &str, stack: &mut Stack) {
//     match op {
//         "+" => {
//             let right = stack.pop().unwrap();
//             let left = stack.pop().unwrap();
//             let result = left + right;
//             stack.push(result.unwrap());
//         },
//         "-" => {
//             let right = stack.pop().unwrap();
//             let left = stack.pop().unwrap();
//             let result = left - right;
//             stack.push(result.unwrap());
//         },
//         _ => println!("Unknown operation"),
//     }
// }


// pub trait Operations{
//     fn execute_operation(op: &str, stack: &mut Stack);
// }
//
// impl Operations for Token {
//     pub fn execute_operation(op: &str, stack: &mut Stack) {
//         match op {
//             "+" => {
//                 if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
//
//                     match left + right {
//                         Ok(result) => stack.push(result),
//                         Err(e) => println!("Error: {}", e),
//                     }
//                 } else {
//                     println!("Error: Stack underflow");
//                 }
//             },
//             // Add other operations and handle different token types similarly
//             _ => println!("Unknown operation"),
//         }
//     }
// }






// pub fn execute_operation(op: &str, stack: &mut Stack) {
//     match op {
//         "+" => {
//             let right = stack.pop().unwrap();
//             let left = stack.pop().unwrap();
//             // Assume both are integers for simplicity; handle other cases and errors properly
//             if let (Token::Integer(a), Token::Integer(b)) = (left, right) {
//                 stack.push(Token::Integer(a + b));
//             }
//         },
//         // Implement other operations
//         _ => (),
//     }
// }






// før
// pub fn interpret(tokens: Vec<Token>) -> Vec<Value> {
//     let mut stack: Vec<Value> = Vec::new();
//     for token in tokens {
//         match token {
//             Token::Integer(num) => stack.push(Value::Integer(num)),
//             Token::Float(num) => stack.push(Value::Float(num)),
//             Token::Arithmetic(op) => execute_operation(&op, &mut stack),
//             // Handle other tokens appropriately
//             _ => (),
//         }
//     }
//     stack
// }
//
// pub fn execute_operation(op: &str, stack: &mut Vec<Value>) {
//     match op {
//         "+" => {
//             let right = stack.pop().unwrap();
//             let left = stack.pop().unwrap();
//             // Assume both are integers for simplicity; handle other cases and errors properly
//             if let (Value::Integer(a), Value::Integer(b)) = (left, right) {
//                 stack.push(Value::Integer(a + b));
//             }
//         },
//         // Implement other operations
//         _ => (),
//     }
// }








// pub fn interpret(commands: Vec<&str>) {
//     for command in commands {
//         println!("Executing command: {}", command);
//         // Add more logic here to handle different commands
//     }
// }




// pub fn interpret(commands: Vec<Token>) {
//     for command in commands {
//         println!("Executing command: {:?}", command);
//         // Add more logic here to handle different commands
//     }
// }









// //  før jeg skiftet
// pub fn execute(stack: &mut Stack<i32>, operation: &str) {
//     match operation {
//         "+" => {
//             let right = stack.pop().unwrap();
//             let left = stack.pop().unwrap();
//             stack.push(left + right);
//         },
//         "-" => {
//             let right = stack.pop().unwrap();
//             let left = stack.pop().unwrap();
//             stack.push(left - right);
//         },
//         "dup" => {
//             if let Some(top) = stack.top() {
//                 stack.push(*top);
//             }
//         },
//         _ => println!("Unknown operation"),
//     }
// }
