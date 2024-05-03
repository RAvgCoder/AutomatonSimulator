use std::fmt;

use crate::parser::ParserError;
use crate::parser::ParserError::{
    MissingObjSeparator, NoObjName, ObjNameMismatch, ObjNameNotFound, ObjNameOverFlow,
    ObjNameSyntaxErr, OutOfInput, ScopeError,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub(in crate::parser) enum Separator {
    COMMA = b',',
    COLUMN = b':',
}

impl Into<char> for Separator {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub(in crate::parser) enum Scope {
    Box_Bracket = b'[',
    Curly_Bracket = b'{',
}

impl Scope {
    pub fn closing(&self) -> char {
        match self {
            Scope::Box_Bracket => ']',
            Scope::Curly_Bracket => '}',
        }
    }
}

impl Into<char> for Scope {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutOfInput(err_message) => write!(f, "OutOfInput {}", err_message),
            NoObjName(err_message) => write!(f, "NoObjName {}", err_message),
            ObjNameSyntaxErr(err_message) => write!(f, "ObjNameSyntaxErr {}", err_message),
            ObjNameNotFound(err_message) => write!(f, "ObjNameNotFound {}", err_message),
            ObjNameOverFlow(err_message) => write!(f, "ObjNameOverFlow {}", err_message),
            ObjNameMismatch(err_message) => write!(f, "ObjNameMismatch {}", err_message),
            MissingObjSeparator(err_message) => write!(f, "MissingObjSeparator {}", err_message),
            ScopeError(err_message) => write!(f, "NoScopeFound {}", err_message),
        }
    }
}

impl State {
    pub fn from_string(s: &str) -> Option<State> {
        match s.to_lowercase().as_str() {
            "type" => Some(State::Type),
            "dfa" => Some(State::AutomatonType),
            "nfa" => Some(State::AutomatonType),
            "pda" => Some(State::AutomatonType),
            "transitions" => Some(State::Transitions),
            "startstate" => Some(State::StartState),
            "acceptstates" => Some(State::AcceptStates),
            "states" => Some(State::States),
            "bulktests" => Some(State::BulkTests),
            _ => None,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub(in crate::parser) enum State {
    Type,
    AutomatonType,
    Transitions,
    StartState,
    AcceptStates,
    States,
    BulkTests,
}
