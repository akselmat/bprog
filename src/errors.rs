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
    ExpectedListAndQuotation


}

// Represents parser errors.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    IncompleteString,
    IncompleteList,
    IncompleteQuotation,
    // nyy
    UnexpectedEndOfInput,
}