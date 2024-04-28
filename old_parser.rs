



// this works great:
// fn nest<'a>(current: &mut Vec<Token>, level: &mut usize, index: &mut usize, tokens: &[String]) -> Result<(), ParserError> {
//     while *index < tokens.len() {
//         match tokens[*index].as_str() {  // Convert String to &str for comparison
//             "]" => {
//                 if *level == 0 {
//                     return Err(ParserError::UnmatchedClosingBracket);
//                 }
//                 *index += 1;
//                 *level -= 1;
//                 return Ok(());
//             },
//             "[" => {
//                 *index += 1;
//                 *level += 1;
//                 let mut new_current = vec![];
//                 crate::parser::nest(&mut new_current, level, index, tokens)?;
//                 current.push(Token::Block(new_current));
//             },
//             _ => {
//                 if let Ok(num) = tokens[*index].parse::<i64>() {
//                     current.push(Token::Int(num));
//                     *index += 1;
//                 }
//             }
//         }
//     }
//     if *level != 0 {
//         Err(ParserError::IncompleteQuotation)
//     } else {
//         Ok(())
//     }
// }



// pub fn parse(&mut self) -> Result<Vec<Token>, String> {
//     // self.tokens = vec!["[", "1", "2", "3", "]"];
//     // self.index = 0;
//     // // Then this:
//     // println!("{}", &self.tokens[self.index]);
//     // [
//
//     while self.index < self.tokens.len() {
//         let token = &self.tokens[self.index].clone();
//         match token.as_str() {  //
//             "[" => self.start_new_block(),
//             "]" => self.end_current_block()?,
//             "\"" => {self.create_string(token)?
//             },
//             // _ if is_alphabetic("dsfsd") => {
//                 // tokens.push(Token::Identifier(word.to_string()));
//             // },
//             _ => self.process_token(token)?,
//         }
//         self.index += 1;
//     }
//     if !self.content_stack.is_empty() {
//         return Err("Unmatched brackets".to_string());
//     }
//     Ok(self.result.clone())
// }



// pub fn split_whitespace_except_quotation(input: &str) -> Vec<String> {
//     let mut result = Vec::new();
//     let mut current_word = String::new();
//     let mut inside_string = false;
//
//     for ch in input.chars() {
//         if ch == '\"' {
//             // Toggle the inside_string flag
//             inside_string = !inside_string;
//             // current_word.push(ch); // Remove or comment out this line if you don't want quotes in the output
//         } else if !inside_string && ch.is_whitespace() {
//             // If outside quotes and it’s a whitespace
//             if !current_word.is_empty() {
//                 // Add the current word to results if it's not empty and then reset
//                 result.push(current_word.clone());
//                 current_word.clear();
//             }
//             // Note: No need to handle whitespace further; just skip it
//         } else {
//             // If outside quotes and it's not a whitespace, or if inside quotes
//             current_word.push(ch);
//         }
//     }
//     result
// }


// pub fn split_whitespace_except_quotation(input: &str) -> Vec<String> {
//     let mut result = Vec::new();
//     let mut current_word = String::new();
//     let mut inside_string = false;
//
//     for ch in input.chars() {
//         if ch == '\"' {
//             // Toggle the inside_string flag
//             inside_string = !inside_string;
//             current_word.push(ch); // Remove or comment out this line if you don't want quotes in the output
//         } else if !inside_string && ch.is_whitespace() {
//             // If outside quotes and it’s a whitespace
//             if !current_word.is_empty() {
//                 // Add the current word to results if it's not empty and then reset
//                 result.push(current_word.clone());
//                 current_word.clear();
//             }
//             // Note: No need to handle whitespace further; just skip it
//         } else {
//             // If outside quotes and it's not a whitespace, or if inside quotes
//             current_word.push(ch);
//         }
//     }
//     result
// }






// fn parse_tokens(chars: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) {
//     while let Some(&ch) = chars.peek() {
//         match ch {
//             _ if ch.is_whitespace() => { chars.next(); }, // Skip whitespace
//             // ' ' | '\n' | '\t' => { chars.next(); }, // Skip whitespace
//             '0'..='9' => {
//                 if let Some(token) = parse_number(chars) {
//                     tokens.push(token);
//                 }
//             }, // Parse numbers
//             '+' | '-' | '*' | '/' => tokens.push(parse_operator(chars)), // Parse operators
//             '{' => {
//                 if let Some(token) = parse_block(chars) {
//                     tokens.push(token);
//                 }
//             }, // Parse blocks
//             _ => { chars.next(); }, // Ignore unexpected characters
//         }
//     }
// }



// impl Parser {
//     pub fn new(input: &str) -> Self {
//         Self {
//             result: vec![],
//             tokens: input.split_whitespace().map(String::from).collect(),
//             content_stack: vec![],
//             index: 0,
//         }
//     }
//     pub fn parse(&mut self) -> Result<Vec<Token>, String> {
//         while self.index < self.tokens.len() {
//             let token = &self.tokens[self.index];
//             match token.as_str() {
//                 "[" => self.start_new_block(),
//                 "]" => self.end_current_block()?,
//                 _ => self.process_token(token)?,
//             }
//             self.index += 1;
//         }
//         if !self.content_stack.is_empty() {
//             return Err("Unmatched brackets".to_string());
//         }
//         Ok(self.result.clone())
//     }



// pub fn parse(&mut self) -> Result<Vec<Token>, String> {
//     while self.index < self.tokens.len() {
//         let token = &self.tokens[self.index];
//         match token.as_str() {
//             // "\"" => make_string(index, words),
//             "\"" => {
//                 // create_string();
//                 // Start a new content block
//                 // self.content_stack.push(vec![]);
//             },
//
//             "[" => {
//                 // Start a new content block
//                 self.content_stack.push(vec![]);
//             },
//             "]" => {
//                 self.end_block()?;
//                 // // End the current content block and add it to result or upper block
//                 // if let Some(finished_block) = self.content_stack.pop() {
//                 //     // returs last vector
//                 //     if let Some(last) = self.content_stack.last_mut() {
//                 //         // if that last vec is last and only vec in stack then:
//                 //         last.push(Token::Block(finished_block));
//                 //     } else {
//                 //         // not last vector then push to result
//                 //         self.result.push(Token::Block(finished_block));
//                 //     }
//                 // } else {
//                 //     return Err("Mismatched brackets".to_string());
//                 // }
//             },
//             _ => {
//                 // Process numbers and other tokens
//                 // if '1' -> is i64
//                 if let Ok(num) = token.parse::<i64>() {
//                     // get the current|last reference to vec[] inside [ ]
//                     if let Some(current) = self.content_stack.last_mut() {
//                         // push integer inside that current vector
//                         current.push(Token::Integer(num));
//                         // now: content_stack = [vec![Token::Integer(1)]].
//                     } else {
//                         // last_mut() is called to see if there's a current block to add tokens to.
//                         // Since content_stack is empty (no open blocks), this call returns None.
//                         self.result.push(Token::Integer(num));
//                     }
//                 } else {
//                     return Err(format!("Unknown token: {}", token));
//                 }
//             }
//         }
//         self.index += 1;
//     }
//     if !self.content_stack.is_empty() {
//         return Err("Unmatched brackets".to_string());
//     }
//     Ok(self.result.clone())
// }

// fn end_current_block(&mut self) -> Result<(), String> {
//     // End the current content block and add it to result or upper block
//     if let Some(finished_block) = self.content_stack.pop() {
//         // returs last vector
//         if let Some(last) = self.content_stack.last_mut() {
//             // if that last vec is last and only vec in stack then:
//             last.push(Token::Block(finished_block));
//         } else {
//             // not last vector then push to result
//             self.result.push(Token::Block(finished_block));
//         }
//     } else {
//         return Err("Mismatched brackets".to_string());
//     }
// }

// }



// #[derive(Debug)]
// pub struct Parser {
//     result: Vec<Token>,
//     tokens: Vec<String>,
//     content: Vec<Token>, // my own
//     index: usize,
//     in_content: bool,
// }
// impl Parser {
//     pub fn new(input: &str) -> Self {
//         Self {
//             result: vec![],
//             tokens: input.split_whitespace().map(String::from).collect(),
//             content: vec![],
//             index: 0,
//             in_content: false,
//         }
//     }
//     pub fn parse(&mut self) -> Result<Vec<Token>, String> {
//         let mut results = Vec::new();
//         while self.index < self.tokens.len() {
//             match (self.tokens[self.index].as_str(), self.in_content) {
//                 ("[", false) => {
//                     self.create_col();
//                     // self.result.push(self.create_col()?)
//                 },
//                 ("]", true)  => {
//                     self.result.push(Token::Block(self.content));
//                     // self.result.push(self.?)
//                 },
//                 // if numbers
//                 _ => {
//                     if let (Ok(num), _check) = (self.tokens[self.index].parse::<i64>(), true) { // if inside of content:
//                         // push inside the content [ ]
//                         self.content.push(Token::Integer(num));
//                     } if let (Ok(num), _check) = (self.tokens[self.index].parse::<i64>(), false){ // if not:
//                         // push it to the result
//                         results.push(Token::Integer(num));
//                     } else {
//                         return Err(format!("Unknown token: {}", self.tokens[self.index]));
//                     }
//                 }
//             }
//             self.index += 1; // Move to the next token
//         }
//         Ok(results)
//     }
//
//     pub fn create_col(&mut self) {
//         self.in_content = true; // we are know in content
//     }



// if let Ok(num) = word.parse::<i64>() {
//     tokens.push(Token::Integer(num));
// } else if let Ok(num) = word.parse::<f64>(){
//     tokens.push(Token::Float(num));
// } else if is_arithmetic(word) {
//     tokens.push(Token::Arithmetic(word.to_string()));
// } else {
//     tokens.push(Token::String(word.to_string()));  // Default case
//     // tokens.push(Token::Identifier(word.to_string()));
// }


// pub fn create_col(&mut self) -> Result<Token, String> {
//     self.in_content += 0; // we are know in content
//     self.index += 1; // Move past '['
//     // let mut contents = Vec::new();
//     // self.parse();
//
//
//
//
//     // while self.index < self.tokens.len() && self.tokens[self.index] != "]" {
//     //     if let Ok(num) = self.tokens[self.index].parse::<i64>() {
//     //         contents.push(Token::Integer(num));
//     //     } else {
//     //         return Err(format!("Error parsing number: {}", self.tokens[self.index]));
//     //     }
//     //     self.index += 1;
//     // }
//     // if self.index == self.tokens.len() || self.tokens[self.index] != "]" {
//     //     return Err("Missing ']' in block".to_string());
//     // }
//     // self.index += 1; // Move past ']'
//     // Ok(Token::Block(contents))
//
// }



// pub fn parse(&mut self) -> Result<Vec<Token>, String> {
//     let mut results = Vec::new();
//     while self.index < self.tokens.len() {
//         match self.tokens[self.index].as_str() {
//             "[" => {
//                 self.index += 1; // Move past '['
//                 results.push(self.parse_block()?);
//             },
//             _ => {
//                 if let Ok(num) = self.tokens[self.index].parse::<i64>() {
//                     results.push(Token::Integer(num));
//                 } else {
//                     return Err(format!("Unknown token: {}", self.tokens[self.index]));
//                 }
//                 self.index += 1; // Move to the next token
//             }
//         }
//     }
//     Ok(results)
// }

// pub fn parse_block(&mut self) -> Result<Token, String> {
//     let mut contents = Vec::new();
//     while self.index < self.tokens.len() && self.tokens[self.index] != "]" {
//         if let Ok(num) = self.tokens[self.index].parse::<i64>() {
//             contents.push(Token::Integer(num));
//         } else {
//             return Err(format!("Error parsing number: {}", self.tokens[self.index]));
//         }
//         self.index += 1;
//     }
//     if self.index == self.tokens.len() || self.tokens[self.index] != "]" {
//         return Err("Missing ']' in block".to_string());
//     }
//     self.index += 1; // Move past ']'
//     Ok(Token::Block(contents))
// }
// }



// #[derive()]
// pub struct blocks_struct{
//     // pub input: String,
//     pub parsed: Vec<&'static str>,
//     pub tokens: Vec<Token>,
//     pub content: Vec<Token>,
//     pub index: usize,
// }
//
//
// impl blocks_struct {
//     pub fn new(index: usize) -> blocks_struct {
//         // blocks_struct { input: String::new(), tokens: vec![], content: vec![], index:0 }
//         blocks_struct { parsed: vec![], tokens: vec![], content: vec![], index:0 }
//     }
//     fn parse<'a>(input: &'static str) -> Vec<&'static str> {
//         input.split_whitespace().collect()
//     }
//
//     fn parse_tokens(mut self) -> self {
//         let mut result = Vec::new();
//         let mut i = 0;
//         let mut couter = 0;
//         let ValidBalancedParentheses = false;
//         while i < self.parsed.len(){
//             match self.parsed[i] {
//                 "[" => {
//                     // go to func create_collection
//                     let list_collec = self.create_collection(&self.parsed[self.index..]); // send the rest of slice from i
//                 }
//                 _ => {}
//                 // _ => { // Handle numbers and possibly other literals
//             //         if let Ok(num) = self.parsed[i].parse::<i64>() {
//             //             result.push(Token::Integer(num));
//             //         } else {
//             //             return Err(format!("Unknown token: {}", self.parsed[i]));
//             //         }
//             //     }
//             // }
//             // self.index += 1;
//         }
//         result
//     }
//     fn create_collection(self) -> self {
//         let mut parenthese_couter = 1;
//         let mut content: Vec<_> = Vec::new();
//         let mut i: usize = 0;
//
//         while i < self.parsed.len() && self.parsed[i]!= "]" {
//             parenthese_couter +=1;
//             // parse the token inside the list
//             self.parse_tokens(&self.parsed[i+1..]);
//         }
//
//         if i == self.parsed.len() || self.parsed[i] != "]" {
//             return Err("Missing ']' in block".to_string());
//         }
//     }
//
// }




// pub fn parse_tokens(tokens: &[&str]) -> Result<Vec<Token>, ParserError> {
//     let mut result = Vec::new();
//     let mut i = 0;
//     let mut couter = 0;
//     let ValidBalancedParentheses = false;
//     while i < tokens.len(){
//         match tokens[i] {
//             "[" => {
//                 // go to func create_collection
//                 let list_collec = create_collection(&tokens[i+1..]); // send the rest of slice from i
//             }
//             _ => { // Handle numbers and possibly other literals
//                 if let Ok(num) = tokens[i].parse::<i64>() {
//                     result.push(Token::Integer(num));
//                 } else {
//                     return Err(format!("Unknown token: {}", tokens[i]));
//                 }
//             }
//         }
//         i += 1;
//     }
//     Ok(result)
// }
// Result<(Vec<Token>, usize), String>
// fn create_collection(tokens: &[&str] ) -> Result<Vec<Token>, String> {
//     let mut parenthese_couter = 1;
//     let mut content: Vec<_> = Vec::new();
//     let mut i: usize = 0;
//
//     while i < tokens.len() && tokens[i]!= "]" {
//         parenthese_couter +=1;
//         // parse the token inside the list
//         parse_tokens(&tokens[i+1..]);
//     }
//
//     if i == tokens.len() || tokens[i] != "]" {
//         return Err("Missing ']' in block".to_string());
//     }
// }




// pub fn parse(tokens: &[&str]) -> Result<Vec<Token>, String> {
//     let mut result = Vec::new();
//     let mut i = 0;
//
//     while i < tokens.len() {
//         match tokens[i] {
//             "[" => {
//                 let block_result = parse_block(&tokens[i..])?;
//                 result.push(Token::Block(block_result.0));
//                 i += block_result.1; // Skip past the end of the block
//             },
//             "]" => {
//                 return Err("Unexpected ']'".to_string());
//             },
//             _ => {
//                 // Handle numbers and possibly other literals
//                 if let Ok(num) = tokens[i].parse::<i64>() {
//                     result.push(Token::Integer(num));
//                 } else {
//                     return Err(format!("Unknown token: {}", tokens[i]));
//                 }
//             }
//         }
//         i += 1;
//     }
//
//     Ok(result)
// }

// // parser
// pub fn parse(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let mut chars = input.chars().peekable();
//     parse_tokens(&mut chars, &mut tokens);
//     tokens
// }


// fn parse_tokens(chars: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) {
//     while let Some(&ch) = chars.peek() {
//         match ch {
//             _ if ch.is_whitespace() => { chars.next(); }, // Skip whitespace
//             // ' ' | '\n' | '\t' => { chars.next(); }, // Skip whitespace
//             '0'..='9' => {
//                 if let Some(token) = parse_number(chars) {
//                     tokens.push(token);
//                 }
//             }, // Parse numbers
//             '+' | '-' | '*' | '/' => tokens.push(parse_operator(chars)), // Parse operators
//             '{' => {
//                 if let Some(token) = parse_block(chars) {
//                     tokens.push(token);
//                 }
//             }, // Parse blocks
//             _ => { chars.next(); }, // Ignore unexpected characters
//         }
//     }
// }
//
// fn create_collection(){
//
// }
//
// fn parse_block(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Token> {
//     chars.next(); // consume '{'
//     let mut content = Vec::new();
//     while let Some(&ch) = chars.peek() {
//         if ch == '}' {
//             chars.next(); // consume '}'
//             return Some(Token::Block(content));
//         }
//         parse_tokens(chars, &mut content); // Recursively parse tokens within the block
//     }
//     None // Failed to properly close block
// }
//
// fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Token> {
//     let mut num_str = String::new();
//     while let Some(&ch) = chars.peek() {
//         if ch.is_digit(10) || ch == '.' {
//             num_str.push(ch);
//             chars.next();
//         } else {
//             break;
//         }
//     }
//     if num_str.contains('.') {
//         num_str.parse::<f64>().ok().map(Token::Float)
//     } else {
//         num_str.parse::<i64>().ok().map(Token::Integer)
//     }
// }
//
// fn parse_operator(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
//     let op = chars.next().unwrap().to_string(); // Safely assuming operator presence here
//     Token::Arithmetic(op)
// }




// pub fn parse(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let words = input.split_whitespace();
//
//     let mut chars = input.chars().peekable();
//     // let mut current = String::new();
//     // let mut in_quotes = false;
//
//     while let Some(&ch) = chars.peek() {
//         match ch {
//             '[' => tokens.push(parse_block(&mut chars)),
//             // '[' => tokens.push(parse_list(&mut chars)?),
//             // '{' => tokens.push(parse_quotation(&mut chars)?),
//             _ if ch.is_whitespace() => { chars.next(); },
//             // _ => return Err(ParserError::IncompleteString),  // Example error
//             _ => {}
//         }
//     }
//     tokens
//
//
//
//     // Now, let's consume the next item
//     // if let Some(&next_value) = chars.peek() {
//     //     // Check if the next value is '[', and if so, consume it
//     //     if next_value == '[' {
//     //         println!("Consuming: {}", chars.next().unwrap());
//     //     }
//     // }
//
//     // [ 1 2 3 ]
//     // for word in words {
//     //     while let
//         // match word {
//         //     // "\"" =>
//         //     "[" => ,
//         //     // "[" => tokens.push(Token::OpenBracket),
//         //     // "]" => tokens.push(Token::CloseBracket),
//         //     // "{" => tokens.push(Token::Block()),
//         //     // "}" => tokens.push(Token::CloseBrace),
//         //     _ => {
//         //         // if let Ok(ch) = word.parse::<char>() {
//         //         //
//         //         // }
//         //         if let Ok(num) = word.parse::<i64>() {
//         //             tokens.push(Token::Integer(num));
//         //         } else if let Ok(num) = word.parse::<f64>(){
//         //             tokens.push(Token::Float(num));
//         //         } else if is_arithmetic(word) {
//         //             tokens.push(Token::Arithmetic(word.to_string()));
//         //         } else {
//         //             tokens.push(Token::String(word.to_string()));  // Default case
//         //             // tokens.push(Token::Identifier(word.to_string()));
//         //         }
//         //     }
//         // }
//     // }
//     //
//     // tokens
// }

fn is_arithmetic(word: &str) -> bool {
    // let arithmetic = &["+","-","*","/"];
    // arithmetic.contains(&word)
    matches!(word, "+" | "-" | "*" | "/")
}





// pub fn parse(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let words = input.split_whitespace();
//
//     for word in words {
//         match word {
//             // "{" => tokens.push(Token::OpenBrace),
//             // "}" => tokens.push(Token::CloseBrace),
//             // "[" => tokens.push(Token::OpenBracket),
//             // "]" => tokens.push(Token::CloseBracket),
//             _ => {
//                 if let Ok(num) = word.parse::<i64>() {
//                     tokens.push(Token::Integer(num));
//                 } else if let Ok(num) = word.parse::<f64>(){
//                     tokens.push(Token::Float(num));
//                 } else if is_arithmetic(word) {
//                     tokens.push(Token::Arithmetic(word.to_string()));
//                 } else {
//                     tokens.push(Token::String(word.to_string()));  // Default case
//                     // tokens.push(Token::Identifier(word.to_string()));
//                 }
//             }
//         }
//     }
//
//     tokens
// }





// pub fn parse(input: &str) -> Vec<Token> {
//     input.split_whitespace().map(|word| match word {
//         "+" | "-" | "*" | "/" => Token::Arithmetic(word.to_string()),
//         _ if word.parse::<i64>().is_ok() => Token::Integer(word.parse().unwrap()),
//         _ if word.parse::<f64>().is_ok() => Token::Float(word.parse().unwrap()),
//         // Add more parsing rules for other token types
//         _ => Token::String(word.to_string()),  // Default case
//     }).collect()
// }




// pub fn parse(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let words = input.split_whitespace();
//
//     for word in words {
//         match word {
//             "{" => tokens.push(Token::OpenBrace),
//             "}" => tokens.push(Token::CloseBrace),
//             "[" => tokens.push(Token::OpenBracket),
//             "]" => tokens.push(Token::CloseBracket),
//             _ => {
//                 if let Ok(num) = word.parse::<i32>() {
//                     tokens.push(Token::Number(num));
//                 } else if is_operator(word) {
//                     tokens.push(Token::Operator(word.to_string()));
//                 } else {
//                     tokens.push(Token::Identifier(word.to_string()));
//                 }
//             }
//         }
//     }
//
//     tokens
// }
// fn is_operator(word: &str) -> bool {
//     matches!(word, "+" | "-" | "*" | "/")
// }























// pub fn parse(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let words = input.split_whitespace();
//
//     for word in words {
//         match word {
//             "{" => tokens.push(Token::OpenBrace),
//             "}" => tokens.push(Token::CloseBrace),
//             "[" => tokens.push(Token::OpenBracket),
//             "]" => tokens.push(Token::CloseBracket),
//             _ => {
//                 if let Ok(num) = word.parse::<i32>() {
//                     tokens.push(Token::Number(num));
//                 } else if is_operator(word) {
//                     tokens.push(Token::Operator(word.to_string()));
//                 } else {
//                     tokens.push(Token::Identifier(word.to_string()));
//                 }
//             }
//         }
//     }
//
//     tokens
// }
//
// fn is_operator(word: &str) -> bool {
//     matches!(word, "+" | "-" | "*" | "/")
// }





// // ny
// pub fn parse(input: &str) -> Vec<String> {
//     input.split_whitespace().map(String::from).collect()
// }
