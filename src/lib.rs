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



// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// enum BracketType {
//     List,
//     Quotation,
//     Undecided,
// }
//
// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// struct BracketContext {
//     list_level: usize,
//     quotation_level: usize,
//     last_opened: BracketType,
// }
//
// impl BracketContext {
//     fn new() -> Self {
//         BracketContext {
//             list_level: 0,
//             quotation_level: 0,
//             last_opened: BracketType::Undecided,
//         }
//     }
//
//     fn open_bracket(&mut self, bracket: BracketType) {
//         match bracket {
//             BracketType::List => {
//                 self.list_level += 1;
//                 self.last_opened = BracketType::List;
//             },
//             BracketType::Quotation => {
//                 self.quotation_level += 1;
//                 self.last_opened = BracketType::Quotation;
//             },
//             _=>{}
//         }
//     }
//
//     fn close_bracket(&mut self, bracket: BracketType) -> Result<(), ParserError> {
//         match bracket {
//             BracketType::List => {
//                 if self.list_level == 0 || self.last_opened != BracketType::List {
//                     return Err(ParserError::IncompleteList);
//                 }
//                 self.list_level -= 1;
//             },
//             BracketType::Quotation => {
//                 if self.quotation_level == 0 || self.last_opened != BracketType::Quotation {
//                     return Err(ParserError::IncompleteQuotation);
//                 }
//                 self.quotation_level -= 1;
//             },
//             _ => {}
//         }
//         Ok(())
//     }
//
//
//
//     // fn is_balanced(&self) -> Result<(), ParserError> {
//         // if self.list_level == 0 && self.quotation_level == 0 {
//         //     Ok(())
//         // } else {
//         //     // Check each type of bracket individually and report specific unbalance.
//         //     if self.list_level > self.quotation_level {
//         //         Err(ParserError::IncompleteList)
//         //     } else { // Implicitly, self.quotation_level > 0
//         //         Err(ParserError::IncompleteQuotation)
//         //     }
//         // }
//
//         // if self.list_level == 0 && self.quotation_level == 0 {
//         //     Ok(())
//         // } else {
//         //     if self.list_level > 0 || self.quotation_level > 0 {
//         //         if self.list_level > self.quotation_level {
//         //             Err(ParserError::IncompleteList)
//         //         } else {
//         //             Err(ParserError::IncompleteQuotation)
//         //         }
//         //     } else {
//         //         Err(IncompleteQuotation)
//         //     }
//         // }
//     // }
//
//     fn is_balanced(&self) -> Result<(), ParserError> {
//         if self.list_level != 0 && self.quotation_level != 0 {
//             Err(ParserError::UnbalancedBrackets)
//         } else {
//             Ok(())
//         }
//     }
// }