allternativ:

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

