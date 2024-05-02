#![allow(unused)]
use std::env;
use std::fmt::Debug;
use std::io::{self, Write}; // for input/output operations
extern crate bprog;
use bprog::stack::Stack;
use bprog::{parser::*,interpreter::*};
use std::fmt;




fn main() {
    let mut interpreter = Interpreter::new(Vec::new());
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("NORMAL");

    match mode {
        "repl" => interpreter.run_repl(), // Start the REPL using the method
        // "normal" => normal_mode(),
        _ => println!("Unknown mode, use 'REPL' or 'NORMAL'"),
    }
}



// fn main() {
//     // let input = " 5 3 /  ";
//     // let input = " 3 4 + 4 5 + + ";
//
//     // let input = " [ False [ ] True [ 1 2 ] ] ";
//     // let input = " [ 1 2 ] 99 ";
//
//
//     // let input = " \" 23  \" parseInteger ";
//     // let input = " 23 ";
//
//     // let input = "  [ 99 ] [ 2 3 ] append ";
//     // let input = "  [ 1 2 3 ] ";
//     // let input = "  { 1 2 + }    ";
//     // let input = " age 10 := age ";
//
//     // let input = " age 20.0 :=  age  ";
//     // let input = " name \" Mariusz \" := name ";
//     // let input = " { 20 10 + } ";
//     // let input = " 1 2 { + } ";
//     // let input = " 1 2 { + } exec ";
//     // let input = " inc { 1 + } fun 1 inc ";
//     // let input = " inc { 9 + } fun inc ";
//     // let input = " 1 { 9 + } exec ";
//     // let input = " inc { 2 + } fun 1 inc ";
//     // let input = " inc { 2 + } fun 1 inc ";
//     // let input = "age 20 := [ 10 [ 11 [ age ] ] 99 ]";
//     // let input = " 1 2 3  ";
//
//
//     // nYYY!!!
//     let input = " [ 1 2 3 ] map { 2 * }  ";
//     let mut parser = Parser::new(input);
//
//     match parser.parse() {
//         Ok(tokens) => {
//             println!("Parser output: {:?}", tokens);
//
//             let mut interpreter = Interpreter::new(tokens);
//             match interpreter.interpret() {
//                 Ok(output) => {
//                     println!("Interpreter output: {:?}", output);
//                     println!("Final: {}", output.iter()
//                     .map(|token| token.to_string())
//                     .collect::<Vec<_>>()
//                     .join(" "));
//                 },
//                 Err(e) => println!("Error during interpretation: {:?}", e),
//             }
//         },
//         Err(e) => println!("Error during parsing: {:?}", e),
//     }
// }

