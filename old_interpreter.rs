



// interpretro before recursive
// interpretro before recursive
// interpretro before recursive
// interpretro before recursive
// interpretro before recursive
// pub fn interpret(tokens: Vec<Token>) -> Result<Vec<Token>, ProgramError> {
//     let mut stack = Stack::new();
//     let mut symbols: HashMap<String, Token> = HashMap::new();
//
//     for token in tokens {
//         match token.clone() {
//             Token::Symbol(sym) if sym == "exec" => {
//                 execute_block(&mut stack)?;
//             }
//             Token::Symbol(symbol) => {
//                 handle_symbol(&symbol, &mut symbols, &mut stack)?;
//             },
//             Token::Int(_) | Token::Float(_) | Token::Bool(_) |
//             Token::String(_) | Token::Bool(_) | Token::Block(_) => {
//                 stack.push(token);
//             },
//             Token::List(items) => {
//                 let evaluated_list = construct_list(&items, &symbols)?;
//                 stack.push(evaluated_list);
//             },
//             Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => {
//                 execute_operation(&op, &mut stack)?;
//             },
//             _ => return Err(ProgramError::UnsupportedType),
//         }
//     }
//
//     if stack.elements.is_empty() {
//         Err(ProgramError::StackEmpty)
//     } else {
//         Ok(stack.elements)  // Return all remaining elements as a vector
//     }
// }



// // Ny tror den funker like bra
// fn construct_list(items: &[Token], symbols: &HashMap<String, Token>) -> Result<Token, ProgramError> {
//     let mut list_items = Vec::new();
//     for item in items {
//         match item {
//             Token::Symbol(sym) if symbols.contains_key(sym) => {
//                 list_items.push(symbols[sym].clone());
//             },
//             _ => list_items.push(item.clone()),
//         }
//     }
//     Ok(Token::List(list_items))
// }


// fn execute_block(stack: &mut Stack) -> Result<(), ProgramError> {
//     let elements = stack.elements.clone();
//     println!("elements:{:?} ", elements.clone());
//     println!("elements length:{:?} ", elements.len().clone());
//     match elements.len() {
//         3 => {
//             let block_token = stack.pop()?; // operations
//             if let Token::Block(tokens) = block_token {
//                 for token in tokens {
//                     match token.clone() {
//                         Token::Arithmetic(op) => {
//                             println!("op:    {:?} ", op.clone());
//                             execute_operation(&op, stack);
//                         }
//                         _=> return Err(ProgramError::ExpectedQuotation)
//                     }
//                 }
//             } else {
//                 return Err(ProgramError::ExpectedQuotation);
//             }
//         },
//         2 => {
//
//         },
//         1 =>{
//             println!("1 elements");
//         },
//         _=> {
//          return Err(ProgramError::StackEmpty)
//         }
//     }
//
//
//
//
//
//     // let block_token = stack.pop()?;
//     // if let Token::Block(tokens) = block_token {
//     //     for token in tokens {
//     //         match interpret_single_token(token, stack) {
//     //             Ok(_) => continue,
//     //             Err(e) => return Err(e),
//     //         }
//     //     }
//     // } else {
//     //     return Err(ProgramError::ExpectedQuotation);
//     // }
//     Ok(())
// }


// pub fn interpret(tokens: Vec<Token>) -> Result<Vec<Token>, ProgramError> {
//     let mut stack = Stack::new();
//     let mut symbols: HashMap<String, Token> = HashMap::new();
//     interpretor(&tokens, &mut symbols, &mut stack);
//     if stack.elements.is_empty() {
//         Err(ProgramError::StackEmpty)
//     } else {
//         Ok(stack.elements.clone())  // Return all remaining elements as a vector
//     }
// }

// interpretro before recursive
// interpretro before recursive






// funker utmerket!!
// fn construct_list(tokens: &[Token], symbols: &HashMap<String, Token>) -> Result<Token, ProgramError> {
//     let mut list_items = Vec::new();
//     for token in tokens {
//         match token {
//             Token::List(inner_tokens) => {
//                 // Recursively evaluate inner lists
//                 let evaluated_list = construct_list(inner_tokens, symbols)?;
//                 list_items.push(evaluated_list);
//             },
//             Token::Symbol(sym) if symbols.contains_key(sym) => {
//                 // Directly substitute the symbol with its corresponding value
//                 list_items.push(symbols[sym].clone());
//             },
//             _ => list_items.push(token.clone()), // For other tokens, add them as they are
//         }
//     }
//     Ok(Token::List(list_items))
// }




// idea:

// pub fn interpret(tokens: Vec<Token>) -> Result<Vec<Token>, ProgramError> {
//     let mut stack = Stack::new();
//     let mut symbols: HashMap<String, Token> = HashMap::new();
//
//     for token in tokens {
//         interpret_token(&token, &mut stack, &mut symbols)?;
//     }
//
//     if stack.elements.is_empty() {
//         Err(ProgramError::StackEmpty)
//     } else {
//         Ok(stack.elements)  // Return all remaining elements as a vector
//     }
// }
//
//
// fn interpret_token(token: &Token, stack: &mut Stack, symbols: &mut HashMap<String, Token>) -> Result<(), ProgramError> {
//     match token {
//         Token::Symbol(symbol) => {
//             handle_symbol(symbol, symbols, stack)?;
//         },
//         Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) => {
//             stack.push(token.clone());
//         },
//         Token::List(items) => {
//             let evaluated_list = construct_list(items, symbols)?;
//             stack.push(evaluated_list);
//         },
//         // Token::Block(block_tokens) => {
//             // execute_block(block_tokens, stack, symbols)?;
//         // },
//         Token::Block(block_tokens) => {
//             // We just push the block onto the stack without executing
//             stack.push(token.clone());
//         },
//         Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => {
//             execute_operation(op, stack, symbols)?;
//         },
//         // Handle additional token types if there are any
//         _ => return Err(ProgramError::UnsupportedType),
//     }
//     Ok(())
// }

// fn execute_block(block_tokens: &[Token], stack: &mut Stack, symbols: &mut HashMap<String, Token>) -> Result<(), ProgramError> {
//     for token in block_tokens {
//         interpret_token(token, stack, symbols)?; // A function you might need to write that handles individual tokens
//     }
//     Ok(())
// }

// fn execute_operation(op: &str, stack: &mut Stack, symbols: &mut HashMap<String, Token>) -> Result<(), ProgramError> {
//     match op {
//         "+" => binary_op("+", stack),
//         "-" => binary_op("-", stack),
//         "*" => binary_op("*", stack),
//         "/" => binary_op("/", stack),
//         "div" => binary_op("div", stack),
//         "&&" => binary_op("&&", stack),
//         "||" => binary_op("||", stack),
//         ">" => binary_op(">", stack),
//         "<" => binary_op("<", stack),
//         "==" => binary_op("==", stack),
//
//         "not" => unary_op("not", stack),
//
//         "dup" => stack_op("dup", stack),
//         "swap" => stack_op("swap", stack),
//         "pop" => stack_op("pop", stack),
//
//         "exec" => exec_op(stack, symbols),
//         // Add more operations as needed
//         _ => Err(ProgramError::UnknownOperation),
//     }
// }
// fn exec_op(stack: &mut Stack, symbols:&mut HashMap<String, Token>) -> Result<(), ProgramError> {
//     let block_token = stack.pop()?;
//     if let Token::Block(block_tokens) = block_token {
//         for token in block_tokens {
//             interpret_token(&token, stack, symbols)?;
//         }
//         Ok(())
//     } else {
//         Err(ProgramError::ExpectedQuotation)
//     }
// }


// allternativ:

pub fn interpret(tokens: Vec<Token>) -> Result<Vec<Token>, ProgramError> {
    let mut stack = Stack::new();
    let mut symbols: HashMap<String, Token> = HashMap::new();

    interpretor(&tokens, &mut symbols, &mut stack)?;

    if stack.elements.is_empty() {
        Err(ProgramError::StackEmpty)
    } else {
        Ok(stack.elements)  // Return all remaining elements as a vector
    }
}


pub fn interpretor(tokens: &[Token], symbols: &mut HashMap<String, Token>, stack: &mut Stack) -> Result<(), ProgramError> {
    for token in tokens {
        match token {
            Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) | Token::Block(_) => {
                stack.push(token.clone());
            },
            Token::List(ref items) => {
                let mut new_stack = Stack::new(); // Create a new stack for the scope of this list
                interpretor(items, symbols, &mut new_stack)?;
                // After interpreting, push the whole list back as a single list token
                stack.push(Token::List(new_stack.elements));
            },
            Token::Symbol(ref symbol) => {
                if let Some(value) = symbols.get(symbol) {
                    stack.push(value.clone());
                } else {
                    // return Err(ProgramError::UnknownSymbol); // Now provides the symbol name
                }
            },
            // Token::Symbol(symbol) => {
            //     // handle_symbol(token.clone(), &mut symbol.clone(), &mut symbols.clone(), &mut stack.clone())?;
            //     if symbol == ":=" {
            //         handle_assignment(symbols, stack)?;
            //     } else {
            //         if let Err(_) = execute_symbol(symbol, symbols, stack){
            //             stack.push(token.clone());
            //         }
            //     }
            // },
            Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => {
                execute_operation(op, stack)?;
            },
            _ => return Err(ProgramError::UnsupportedType),
        }
    }
    Ok(())
}




// pub fn interpret(tokens: Vec<Token>) -> Result<Vec<Token>, ProgramError> {
//     let mut stack = Stack::new();
//     let mut symbols: HashMap<String, Token> = HashMap::new();
//
//     for token in tokens {
//         match token.clone() {
//             Token::Symbol(symbol) => {
//                 handle_symbol(token, &symbol, &mut symbols, &mut stack)?;
//             },
//             Token::Int(_) | Token::Float(_) | Token::Bool(_) |
//             Token::String(_) | Token::Block(_) => {
//                 // Token::String(_) | Token::List(_) | Token::Block(_) => {
//                 stack.push(token);
//             },
//             Token::List(items) => {
//                 let evaluated_list = construct_list(&items, &symbols)?;
//                 stack.push(evaluated_list);
//             },
//             // Token::List(ref items) => {
//             //     let evaluated_list = construct_list(items, &symbols)?;
//             //     stack.push(evaluated_list);
//             // },
//             // Token::List(list_tokens) => {
//             //     for l_token in list_tokens {
//             //         if let Token::Symbol(tok) = l_token {
//             //             execute_symbol(&tok, &mut symbols, &mut stack)?;
//             //         } else {
//             //             stack.push(token.clone());
//             //         }
//             //     }
//             // },
//
//             Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => {
//                 execute_operation(&op, &mut stack)?;
//             },
//             _ => return Err(ProgramError::UnsupportedType),
//         }
//     }
//
//     if stack.elements.is_empty() {
//         Err(ProgramError::StackEmpty)
//     } else {
//         Ok(stack.elements)  // Return all remaining elements as a vector
//     }
// }



// // stack operations
// fn stack_op(op: &str, stack: &mut Stack) -> Result<(), ProgramError> {
//     match op {
//         "swap" => stack.swap(),
//         "pop" => {
//             stack.pop()?;  // Pop and discard the top element
//             Ok(())
//         },
//         _ => unreachable!(),
//     }
// }

// pub fn interpret(tokens: Vec<Token>) -> Result<Token, ProgramError> {
//     let mut stack = Stack::new();
//
//     for token in tokens {
//         match token {
//             Token::Int(_) | Token::Float(_) | Token::Bool(_) | Token::String(_) | Token::List(_) => {
//                 stack.push(token);
//             },
//             Token::Arithmetic(op) | Token::LogicalOp(op) | Token::ListOp(op) => execute_operation(&op, &mut stack)?,
//             _=>{
//                 Err(ProgramError::UnsupportedType)?
//             }
//         }
//     }
//
//     // Check that there is exactly one value left on the stack
//     if stack.elements.len() == 1 {
//         Ok(stack.pop()?)
//     } else if stack.elements.is_empty() {
//         Err(ProgramError::StackEmpty)
//     } else {
//         println!("sdfsdfs");
//         Err(ProgramError::ProgramFinishedWithMultipleValues)
//     }
// }

