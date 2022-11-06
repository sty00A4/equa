use crate::lexer::*;
use crate::parser::*;
use crate::value::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Todo(String),
    Char(String),
    UnexpectedToken(Token),
    UnexpectedNode(Node),
    ExpectedToken(Token),
    ExpectedType(Type),
    InvalidBinaryOperation(Token),
    InvalidUnaryOperation(Token),
    IllegalBinaryOperation(Token, Value, Value),
    IllegalUnaryOperation(Token, Value),
    Immutable(String),
    AlreadyDefined(String),
    NotDefined(String),
    TooFewArgs(usize, usize),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Todo(s) => write!(f, "ERROR: todo - {s}"),
            Self::Char(s) => write!(f, "ERROR: unexpected '{s}'"),
            Self::UnexpectedToken(t) => write!(f, "ERROR: unexpected {}", t.name()),
            Self::UnexpectedNode(n) => write!(f, "ERROR: unexpected {n}"),
            Self::ExpectedToken(t) => write!(f, "ERROR: expected {}", t.name()),
            Self::ExpectedType(t) => write!(f, "ERROR: expected type {t}"),
            Self::InvalidBinaryOperation(t) => write!(f, "ERROR: invalid binary operation {}", t.name()),
            Self::InvalidUnaryOperation(t) => write!(f, "ERROR: invalid unary operation {}", t.name()),
            Self::IllegalBinaryOperation(t, v1, v2) => write!(f, "ERROR: illegal binary operation {} between {} and {}",
            t.name(), v1.typ(), v2.typ()),
            Self::IllegalUnaryOperation(t, v) => write!(f, "ERROR: illegal unary operation {} on {}",
            t.name(), v.typ()),
            Self::Immutable(s) => write!(f, "ERROR: {s} is immutable"),
            Self::AlreadyDefined(s) => write!(f, "ERROR: {s} is already defined"),
            Self::NotDefined(s) => write!(f, "ERROR: {s} is not defined"),
            Self::TooFewArgs(expect, recved) => write!(f, "ERROR: expected length of {expect} for the arguments, got length of {recved}"),
        }
    }
}