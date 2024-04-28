#![allow(unused)]
// mod cat;
// use cat::Cat;
// use bprog::Token;
use std::io::{self, Write}; // for input/output operations
// use std::io::{self, BufRead};

mod stack;
mod token;
mod parser;
mod interpreter;
mod errors;

use errors::{ProgramError, ParserError};
use stack::Stack;
use interpreter::{interpret};
use crate::errors::ProgramError::ExpectedEnumerable;
use crate::parser::{Parser, split_into_tokens};


fn main() {
    // let input = " 5 3 /  ";
    // let input = " 3 4 + 4 5 + + ";

    // let input = " [ 1 2 ] \" aksel \" ";
    // let input = " [ 1 2 ] 99 ";

    let input = " [ 1 ] length ";
    println!("string tokens : {:?}", input);
    println!("string tokens split : {:?}", split_into_tokens(input));


    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(_) => {
            // If parsing is successful, get and print the result
            let results = parser.get_result();
             println!("tokens: {:?}", results);

             println!("tokens interpret: {:?}", interpret(results.clone()));
            if let Err(e) = interpret(results){
                 println!("Error during parsing: {:?}", e);
            }

        },
        Err(e) => {
            // If there was an error during parsing, print the error
            println!("Error during parsing: {:?}", e);
        }
    }
}

// fn run(input: &str) {
//     let mut parser = Parser::new(input);
//     // let tokens = parse(input);
//     // let output = interpret(tokens);
//     // println!("Output: {:?}", output);
//     match parser.parse() {
//         Ok(_) => {
//             // If parsing is successful, get and print the result
//             let results = parser.get_result();
//             println!("Parsed tokens: {:?}", results);
//             println!("Parsed tokens: {:?}", interpret(results));
//         },
//         Err(e) => {
//             // If there was an error during parsing, print the error
//             println!("Error during parsing: {:?}", e);
//         }
//     }


// }




// fn main() {
//     // let input = "3 1 -";
//     let input = " [ 2 3 ] ";
//
//
//     run(&input);
// }


// fn run(input: &str) {
//     let tokens = parse(input);
//     let output = interpret(tokens);
//     println!("Output: {:?}", output);
// }




// GHCI
// fn repl() {
//     let stdin = io::stdin();
//     let mut stdout = io::stdout();
//
//     loop {
//         print!("bprog> ");
//         stdout.flush().unwrap(); // Ensure "bprog> " prompt appears immediately
//
//         let mut input = String::new();
//         match stdin.read_line(&mut input) {
//             Ok(0) => break, // EOF reached
//             Ok(_) => run(&input.trim()),
//             Err(error) => println!("Error: {}", error),
//         }
//     }
// }





// fn main() {
//     let input = "3 4 +"; // Example input
//     let commands = parse(input);
//     interpret(commands);
// }

// fn main() {
//     let mut stack = Stack::new();
//     let input = "3 4 +"; // Example input
//     for line in input.lines() {
//         let input = line;
//         let tokens = parse(&input);
//         for token in tokens {
//             if let Ok(num) = token.parse::<i32>() {
//                 stack.push(num);
//             } else {
//                 execute(&mut stack, &token);
//             }
//         }
//         println!("Stack: {:?}", stack.elements);
//     }
// }






// fn main() {
//     let mut stack = Stack::new();
//     let stdin = io::stdin();
//     for line in stdin.lock().lines() {
//         let input = line.unwrap();
//         let tokens = parse(&input);
//         for token in tokens {
//             if let Ok(num) = token.parse::<i32>() {
//                 stack.push(num);
//             } else {
//                 execute(&mut stack, &token);
//             }
//         }
//         println!("Stack: {:?}", stack.elements);
//     }
// }


// fn main() {
//     let input = "3 4 +"; // Example input
//     let commands = parse(input);
//     let op = interpret(commands);
//     execute_operation(op);
//
// }