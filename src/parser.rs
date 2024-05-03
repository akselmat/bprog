#![allow(unused)]

use std::process::id;
use std::vec;
use crate::{BracketContext, BracketType};
use crate::BracketType::{List, Quotation};
use crate::token::Token;
use crate::errors::{ParserError, ProgramError};






#[derive(Debug, Clone, PartialEq, PartialOrd)] // må kanskje endre dette
pub struct Parser {
    parsed_tokens: Vec<Token>,
    raw_tokens: Vec<String>,
    current_index: usize,
    bracket_context: BracketContext,
}

impl Parser  {
    pub fn new(input: &str) -> Self {
        Self {
            parsed_tokens: vec![],
            raw_tokens: split_into_tokens(input),
            current_index: 0,
            bracket_context: BracketContext::new(),
        }
    }

    pub fn parse_tokens(&mut self) -> Result<Vec<Token>, ParserError> {
        match parse_tokens_recursively(&mut self.parsed_tokens, &mut self.bracket_context, &mut self.current_index, &self.raw_tokens) {
            Ok(()) => Ok(self.parsed_tokens.clone()),
            Err(e) => Err(e),
        }
    }
}


fn parse_tokens_recursively<'a>(current: &mut Vec<Token>, context: &mut BracketContext, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    while *index < tokens.len() {
        let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
        // let input = " [ { } ]  ";
        match token {  // Convert String to &str for comparison
            "[" => {
                context.open_bracket(BracketType::List);
                *index += 1;
                let mut new_current = vec![];
                parse_tokens_recursively(&mut new_current, context, index, tokens)?;
                current.push(Token::List(new_current));
            },
            "]" => {
                context.close_bracket(BracketType::List)?;
                *index += 1;
                return Ok(());
            },
            "{" => {
                context.open_bracket(BracketType::Quotation);
                *index += 1;
                let mut block_tokens = vec![];
                parse_tokens_recursively(&mut block_tokens, context, index, tokens)?;
                current.push(Token::Block(block_tokens));
            },
            "}" => {
                context.close_bracket(BracketType::Quotation)?;
                *index += 1;
                return Ok(());
            },
            _ if token.parse::<i128>().is_ok() => create_int(current, index, tokens)?,
            _ if token.parse::<f64>().is_ok() => create_float(current, index, tokens)?,
            "\"" => create_string(current, index, tokens)?,
            "div"|"+"| "-" | "*" | "/"| "swap"|"pop"|"dup"|"parseInteger"|"parseFloat" => is_arithmetic(current, index, tokens)?,
            "True"|"False" => is_bool(current, index, tokens)?,
            "not"|"&&"|"||"|">"|"<"|"==" => is_logical(current, index, tokens)?,
            "head"|"tail"|"empty"|"length"|"append"|"each"|"cons"|"append" => is_list_operations(current, index, tokens)?,
            // "parseInteger"|"parseFloat"|"words" => is_string_parsing(current, index, tokens)?,
            _ => is_symbol(current, index, tokens)?,
        }
    }
    context.is_balanced()
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