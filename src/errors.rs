#![allow(unused)]
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    StackEmpty,
    UnknownSymbol(String), // symbols: a_symbol Note: because there are no restrictions on symbols, anything that is not a reserved keyword in the language can become a valid symbol, and therefore, a function name.
    ExpectedBool,
    ExpectedBoolOrNumber,
    ExpectedEnumerable,
    ExpectedQuotation,
    ExpectedList,
    ExpectedVariable(String),
    DivisionByZero,
    ProgramFinishedWithMultipleValues,
    NumberConversionError,


    // Ny
    UnknownOperation,
    NotEnoughElements,
    UnexpectedToken,
    UnsupportedType,
    UnmatchedClosingBracket,
    IncompleteString,
}

// Represents parser errors.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    IncompleteString,
    IncompleteList,
    IncompleteQuotation,






    // nyy
    UnexpectedEndOfInput, // ny
    UnmatchedQuotes, // ny
    UnmatchedClosingBracket, // ny
    UnexpectedToken, // nyyyy

    StackUnderflow,
    UnsupportedType,
    UnknownOperation,
}