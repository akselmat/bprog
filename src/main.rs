#![allow(unused)]
use std::io::{self, Write}; // for input/output operations
extern crate bprog;
use bprog::{parser::*,interpreter::*};
use bprog::stack::Stack;


fn main() {
    // let input = " 5 3 /  ";
    // let input = " 3 4 + 4 5 + + ";

    // let input = " [ False [ ] True [ 1 2 ] ] ";
    // let input = " [ 1 2 ] 99 ";


    // let input = " \" 23  \" parseInteger ";
    // let input = " 23 ";
    let input = " 10 20 pop ";


    println!("string tokens : {:?}", input);
    let split_tok = split_into_tokens(input);
    println!("string tokens split : {:?}", split_tok);

    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(results) => {
            println!("parser: {:?}", results.clone());
            // println!("tokens interpret: {:?}", interpret(results.clone()).unwrap());
            match interpret(results.clone()) {
                Ok(tokens) => println!("tokens interpret [{}]", tokens.iter()
                        .map(|token| token.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")),
                Err(e) => println!("Error: {:?}", e),
            }
        },
        Err(e) => {
            // If there was an error during parsing, print the error
            println!("Error during parsing: {:?}", e);
        }
    }
    let mut stack = Stack::new();
    let stackkk = stack.elements;
    for v in stackkk {
        println!("stack ellements {:?}", v);
    }


    // match parser.parse() {
    //     Ok(_) => {
    //         // If parsing is successful, get and print the result
    //         let results = parser.get_result();
    //          println!("tokens: {:?}", results);
    //
    //          println!("tokens interpret: {:?}", interpret(results.clone()));
    //         if let Err(e) = interpret(results){
    //              println!("Error during parsing: {:?}", e);
    //         }
    //
    //     },
    //     Err(e) => {
    //         // If there was an error during parsing, print the error
    //         println!("Error during parsing: {:?}", e);
    //     }
    // }
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