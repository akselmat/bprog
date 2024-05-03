use crate::interpreter::*;
use crate::parser::Parser;

pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod stack;
pub mod token;


pub fn t(input: &str) -> String {
    let mut parser = Parser::new(input);

    match parser.parse() {
        Ok(tokens) => {
            let mut interpreter = Interpreter::new(tokens);
            match interpreter.interpret() {
                Ok(stack) => format!("{}", stack.elements.iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<_>>()
                    .join(" "))
                ,
                Err(e) => format!("Error: {:?}", e),
            }
        },
        Err(e) => format!("Error during parsing: {:?}", e),
    }
}