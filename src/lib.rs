use crate::errors::ParserError;
use crate::errors::ParserError::*;
use crate::interpreter::*;
use crate::parser::Parser;

pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod stack;
pub mod token;


pub fn t(input: &str) -> String {
    let mut parser = Parser::new(input);

    match parser.parse_tokens() {
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



#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum BracketType {
    List,
    Quotation,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct BracketContext {
    bracket_stack: Vec<BracketType>,
}

impl BracketContext {
    fn new() -> Self {
        BracketContext {
            bracket_stack: Vec::new(),
        }
    }
    fn open_bracket(&mut self, bracket: BracketType) {
        self.bracket_stack.push(bracket);
    }

    fn close_bracket(&mut self, bracket: BracketType) -> Result<(), ParserError> {
        if bracket == BracketType::List {
            if self.bracket_stack.pop() != Some(BracketType::List) {
                return Err(ParserError::IncompleteList);
            }
        } else {
            if self.bracket_stack.pop() != Some(BracketType::Quotation) {
                return Err(ParserError::IncompleteQuotation);
            }
        }
        Ok(())
    }

    fn is_balanced(&self) -> Result<(), ParserError> {
        if !self.bracket_stack.is_empty() {
            return Err(UnbalancedBrackets);
        }
        Ok(())
    }

}