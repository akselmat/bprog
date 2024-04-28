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
    // pub fn get_current_token(&self) -> Result<&str, ParserError>  {
    //     Ok(self.tokens.get(self.index).ok_or(ParserError::UnexpectedEndOfInput)?)
    // }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        nest(&mut self.result, &mut self.level, &mut self.index, &self.tokens)?;
        Ok(())
    }
}


// !!!!!!!!
// FUNCTIONS
fn nest<'a>(current: &mut Vec<Token>, level: &mut usize, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    while *index < tokens.len() {
        let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
        match token {  // Convert String to &str for comparison
            "]" => close_brackets(level,index)?,
            "[" => create_list(current , level, index, tokens)?,
            _ if token.parse::<i64>().is_ok() => create_int(current, index, tokens)?,
            _ if token.parse::<f64>().is_ok() => create_float(current, index, tokens)?,
            "\"" => create_string(current, index, tokens)?,
            "+"| "-" | "*" | "/" => is_arithmetic(current, index, tokens)?,
            _ => Err(ParserError::UnexpectedToken)?
        }
    }
    validate_brackets(level)
}

fn is_arithmetic(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    let token = tokens.get(*index).ok_or(ParserError::UnexpectedEndOfInput)?.as_str();
    current.push(Token::Arithmetic(token.to_string()));
    *index += 1; // go to next token
    Ok(())
}


fn create_func(current: &mut Vec<Token>, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    if let Ok(num) = tokens[*index].parse::<i64>() {
        current.push(Token::Int(num));
        *index += 1;
    }
    Ok(())
}


fn validate_brackets(level: &mut usize) -> Result<(), ParserError>{
    if *level != 0 {
        Err(ParserError::IncompleteList)
    } else {
        Ok(())
    }
}

fn close_brackets(level: &mut usize, index: &mut usize) -> Result<(), ParserError> {
    if *level == 0 {
        return Err(ParserError::UnmatchedClosingBracket);
    }
    *index += 1;
    *level -= 1;
    return Ok(());
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
    if let Ok(num) = tokens[*index].parse::<i64>() {
        current.push(Token::Int(num));
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

fn create_list<'a>(current: &mut Vec<Token>, level: &mut usize, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
    *index += 1;
    *level += 1;
    let mut new_current = vec![];
    nest(&mut new_current, level, index, tokens)?;
    current.push(Token::Block(new_current));
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
                    result.push(current_word);
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