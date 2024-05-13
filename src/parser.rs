pub mod parser;
mod utils;

pub struct Parser {
    program_iter: String,
    cursor: u32,
}

/// Represents errors that could occur in a parser
#[derive(Debug)]
pub enum ParserError {
    OutOfInput(String),
    NoObjName(String),
    ObjNameSyntaxErr(String),
    ObjNameNotFound(String),
    ObjNameOverFlow(String),
    ObjNameMismatch(String),
    MissingObjSeparator(String),
    ScopeError(String),
    UnknownTransitionName(String),
}

/// Describes a separator used in the parser
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub(in crate::parser) enum Separator {
    COMMA = b',',
    COLUMN = b':',
}


/// Represents an opening scope
/// represented by "{" OR "["
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub(in crate::parser) enum Scope {
    BoxBracket = b'[',
    CurlyBracket = b'{',
}

/// Represents the names that should all automatons share in the json
/// format from the automaton simulator site
#[derive(Debug, PartialOrd, PartialEq)]
pub(in crate::parser) enum SkeletonState {
    Type,
    AutomatonType,
    Transitions,
    StartState,
    AcceptStates,
    States,
    BulkTests,
}