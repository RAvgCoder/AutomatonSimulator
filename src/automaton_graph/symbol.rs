use std::fmt;
use std::fmt::{Display, Formatter};

use crate::automaton_graph::Symbol;

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self.clone()))
    }
}

impl From<Symbol> for char {
    fn from(value: Symbol) -> char {
        match value {
            Symbol::CHAR(c) => c,
            Symbol::EPSILON => 'ϵ',
        }
    }
}
