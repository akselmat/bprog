#![allow(unused)]

use std::process::id;
use std::vec;
use crate::token::Token;
use crate::errors::{ParserError, ProgramError};


#[derive(Debug, Clone, PartialEq, PartialOrd)] // må kanskje endre dette
pub struct Parser {
    result: Vec<Token>,
    tokens: Vec<String>,
    index: usize,
    level: usize,
}

impl Parser  {
    pub fn new(input: &str) -> Self {
        Self {
            result: vec![],
            tokens: split_into_tokens(input),
            index: 0,
            level: 0,
        }
    }
    pub fn get_result(&self) -> Vec<Token> {
        self.result.clone()
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, ParserError> {
        match nest(&mut self.result, &mut self.level, &mut self.index, &self.tokens) {
            Ok(()) => Ok(self.get_result()),
            Err(e) => Err(e),  // Forward the actual error directly
        }
    }
}


// !!!!!!!!
// FUNCTIONS
fn nest<'a>(current: &mut Vec<Token>, level: &mut usize, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    while *index < tokens.len() {
        let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
        match token {  // Convert String to &str for comparison
            "]" => {
                if *level == 0 {
                    return Err(ParserError::IncompleteList);
                }
                *index += 1;
                *level -= 1;
                return Ok(());
            },
            "[" => {
                *index += 1;
                *level += 1;
                let mut new_current = vec![];
                nest(&mut new_current, level, index, tokens)?;
                current.push(Token::List(new_current));
            },
            "{" => {
                *index += 1;
                *level += 1;
                let mut block_tokens = vec![];
                nest(&mut block_tokens, level, index, tokens)?;
                current.push(Token::Block(block_tokens));
            },
            "}" => {
                if *level == 0 {
                    return Err(ParserError::IncompleteQuotation);
                }
                *index += 1;
                *level -= 1;
                return Ok(());
            },
            _ if token.parse::<i128>().is_ok() => create_int(current, index, tokens)?,
            _ if token.parse::<f64>().is_ok() => create_float(current, index, tokens)?,
            "\"" => create_string(current, index, tokens)?,
            "div"|"+"| "-" | "*" | "/"| "swap"|"pop"|"dup" => is_arithmetic(current, index, tokens)?,
            "True"|"False" => is_bool(current, index, tokens)?,
            "not"|"&&"|"||"|">"|"<"|"==" => is_logical(current, index, tokens)?,
            // "head"|"tail"|"empty"|"length"|"append"|"each"|"cons"|"append"|"map" => is_list_operations(current, index, tokens)?,
            "head"|"tail"|"empty"|"length"|"append"|"each"|"cons"|"append" => is_list_operations(current, index, tokens)?,
            // "parseInteger"|"parseFloat"|"words" => is_string_parsing(current, index, tokens)?,
            _ => is_symbol(current, index, tokens)?,
        }
    }
    if *level != 0 {
        Err(ParserError::IncompleteQuotation)
    } else {
        Ok(())
    }
}

fn is_symbol(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
    current.push(Token::Symbol(token.to_string()));
    *index += 1; // go to next token
    Ok(())
}

fn is_list_operations(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
    current.push(Token::ListOp(token.to_string()));
    *index += 1; // go to next token
    Ok(())
}

fn is_logical(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
    current.push(Token::LogicalOp(token.to_string()));
    *index += 1; // go to next token
    Ok(())
}
fn is_bool(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
    if token == "True" {
        current.push(Token::Bool(true));
    } else {
        current.push(Token::Bool(false));
    }
    *index += 1; // Move past the closing quote
    Ok(())
}

fn is_arithmetic(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
    current.push(Token::Arithmetic(token.to_string()));
    *index += 1; // go to next token
    Ok(())
}

fn create_string(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    *index += 1; // Move past the opening quote
    let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();

    *index += 1; // Move past the string content
    let next_token = tokens.get(*index).ok_or(ParserError::IncompleteString)?.as_str();
    if next_token == "\"" {
        *index += 1; // Move past the closing quote
        current.push(Token::String(token.to_string()));
        Ok(())
    } else {
        Err(ParserError::IncompleteString)
    }
}

fn create_int(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    if let Ok(num) = tokens[*index].parse::<i128>() {
        current.extend(vec![Token::Int(num)]);
        *index += 1;
    }
    Ok(())
}

fn create_float(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    if let Ok(num) = tokens[*index].parse::<f64>() {
        current.push(Token::Float(num));
        *index += 1;
    }
    Ok(())
}

pub fn split_into_tokens(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_word = String::new();
    let mut inside_string = false;

    for ch in input.chars() {
        match ch {
            '\"' => {
                // Handle the transition of inside and outside quotes
                if !current_word.is_empty() {
                    // result.push(current_word);
                    result.push(current_word.trim().to_string()); // ny
                    current_word = String::new();
                }
                result.push(ch.to_string());  // Add the quote as a separate token
                inside_string = !inside_string;  // Toggle the inside_string flag
            },
            ' ' if !inside_string => {
                // Complete the current word if it's outside of quotes
                if !current_word.is_empty() {
                    result.push(current_word);
                    current_word = String::new();
                }
            },
            _ => {
                // Add the character to the current word
                current_word.push(ch);
            }
        }
    }

    // Handle any remaining word after the loop
    if !current_word.is_empty() {
        result.push(current_word);
    }

    result
}