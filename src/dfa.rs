use crate::automaton_graph::Automaton;
use crate::parser::Parser;

pub struct DFA<'a> {
    automaton_graph: Option<Automaton<'a>>,
}

impl<'a> DFA<'a> {
    pub fn new(program: String) -> Self {
        DFA {
            automaton_graph: Parser::parse(program)
        }
    }

    pub fn reduce(&self) {}
}