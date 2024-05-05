#![allow(unused)]
use std::env;
use std::fmt::Debug;
use std::io::{self, Write}; // for input/output operations
extern crate bprog;
use bprog::stack::Stack;
use bprog::{parser::*,interpreter::*};
use std::fmt;
use bprog::errors::ProgramError;

fn main() {
    let mut interpreter = Interpreter::new(Vec::new());
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("normal");
    match mode {
        "repl" => interpreter.run_repl_mode(), // Start the REPL using the method
        _ =>  {
            interpreter.run_normal_mode()
        },
    }
}
