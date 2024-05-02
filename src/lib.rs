use crate::interpreter::*;
use crate::parser::Parser;

pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod stack;
pub mod token;



// pub fn t(input: &str ) -> String
// {
//     let mut parser = Parser::new(input);
//     match parser.parse() {
//         Ok(results) => {
//             let result = interpret(results);
//             for v in result{
//                 match v {
//                     Ok(v) => v.to_string(),
//                     Err(e) => format!("Error: {:?}", e),
//                 }
//             }
//         },
//         Err(e) => format!("Error during parsing: {:?}", e),
//     }
// }

// pub fn t(input: &str ) -> String
// {
//     let mut parser = Parser::new(input);
//     match parser.parse() {
//         Ok(results) => {
//             let result = interpret(results);
//             match result {
//                 Ok(token) => token.to_string(),
//                 Err(e) => format!("Error: {:?}", e),
//             }
//         },
//         Err(e) => format!("Error during parsing: {:?}", e),
//     }
// }

pub fn t(input: &str) -> String {
    let mut parser = Parser::new(input);

    match parser.parse() {
        Ok(tokens) => {
            println!("Parser output: {:?}", tokens);
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


    // match parser.parse() {
    //     Ok(tokens) => {
    //         println!("Parser output: {:?}", tokens);
    //         let mut interpreter = Interpreter::new(tokens);
    //         match interpreter.interpret() {
    //             Ok(output) => format!("{}", output.iter()
    //                 .map(|token| token.to_string())
    //                 .collect::<Vec<_>>()
    //                 .join(" "))
    //             ,
    //             Err(e) => format!("Error: {:?}", e),
    //         }
    //     },
    //     Err(e) => format!("Error during parsing: {:?}", e),
    // }


    // match parser.parse() {
    //     Ok(results) => {
    //         match interpretor(results) {
    //             Ok(tokens) => format!("{}", tokens.iter()
    //                     .map(|token| token.to_string())
    //                     .collect::<Vec<_>>()
    //                     .join(" "))
    //             ,
    //             Err(e) => format!("Error: {:?}", e),
    //         }
    //     },
    //     Err(e) => format!("Error during parsing: {:?}", e),
    // }
}

// pub fn t(input: &str) -> String {
//     let mut parser = Parser::new(input);
//     match parser.parse() {
//         Ok(results) => {
//             match interpret(results) {
//                 Ok(tokens) => format!("{}", tokens.iter()
//                     .map(|token| token.to_string())
//                     .collect::<Vec<_>>()
//                     .join(" "))
//                 ,
//                 Err(e) => format!("Error: {:?}", e),
//             }
//         },
//         Err(e) => format!("Error during parsing: {:?}", e),
//     }
// }