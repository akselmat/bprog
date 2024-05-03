use crate::errors::ParserError;
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
    Undecided,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct BracketContext {
    list_level: usize,
    quotation_level: usize,
    last_opened: BracketType,
}

impl BracketContext {
    fn new() -> Self {
        BracketContext {
            list_level: 0,
            quotation_level: 0,
            last_opened: BracketType::Undecided,
        }
    }

    fn open_bracket(&mut self, bracket: BracketType) {
        match bracket {
            BracketType::List => {
                self.list_level += 1;
                self.last_opened = BracketType::List;
            },
            BracketType::Quotation => {
                self.quotation_level += 1;
                self.last_opened = BracketType::Quotation;
            },
            _=>{}
        }
    }

    fn close_bracket(&mut self, bracket: BracketType) -> Result<(), ParserError> {
        match bracket {
            BracketType::List => {
                if self.list_level == 0 || self.last_opened != BracketType::List {
                    return Err(ParserError::MismatchedBracket);
                }
                self.list_level -= 1;
            },
            BracketType::Quotation => {
                if self.quotation_level == 0 || self.last_opened != BracketType::Quotation {
                    return Err(ParserError::MismatchedBracket);
                }
                self.quotation_level -= 1;
            },
            _ => {}
        }
        Ok(())
    }

    fn is_balanced(&self) -> Result<(), ParserError> {
        if self.list_level != 0 && self.quotation_level != 0 {
            Err(ParserError::UnbalancedBrackets)
        } else {
            Ok(())
        }
    }
}