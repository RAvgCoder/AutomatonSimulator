use crate::automaton_graph::Symbol;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Symbol::CHAR(c) => write!(f, "{}", c),
            Symbol::EPSILON => write!(f, "ϵ"),
        }
    }
}
