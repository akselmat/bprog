use crate::interpreter::interpret;
use crate::parser::Parser;

pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod stack;
pub mod token;

pub fn t(input: &str ) -> String
{
    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(results) => {
            let result = interpret(results);
            match result {
                Ok(token) => token.to_string(),
                Err(e) => format!("Error: {:?}", e),
            }
        },
        Err(e) => format!("Error during parsing: {:?}", e),
    }
}