#![allow(unused)]
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    StackEmpty,
    UnknownSymbol,
    ExpectedBool,
    ExpectedBoolOrNumber,
    ExpectedEnumerable,
    ExpectedQuotation,
    ExpectedList,
    ExpectedVariable,
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
    ExpectedListAndQuotation,

    ParserError(ParserError),  // Add this to encapsulate parser errors
}

// Represents parser errors.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    IncompleteString,
    IncompleteList,
    IncompleteQuotation,

    // nyy
    UnexpectedEndOfInput,
    UnbalancedBrackets
}

impl From<ParserError> for ProgramError {
    fn from(err: ParserError) -> Self {
        ProgramError::ParserError(err)
    }
}