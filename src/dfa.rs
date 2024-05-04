use crate::automaton_graph::Automaton;
use crate::parser::Parser;

pub struct DFA {
    automaton_graph: Option<Automaton>,
}

impl DFA {
    pub fn new(program: String) -> Self {
        DFA {
            automaton_graph: Parser::parse(program),
        }
    }

    pub fn reduce(&self) {}
}
