pub mod parser;
mod utils;

pub struct Parser {
    program_iter: String,
    cursor: u32,
}

#[derive(Debug)]
enum ParserError {
    OutOfInput(String),
    NoObjName(String),
    ObjNameSyntaxErr(String),
    ObjNameNotFound(String),
    ObjNameOverFlow(String),
    ObjNameMismatch(String),
    MissingObjSeparator(String),
    ScopeError(String),
}
