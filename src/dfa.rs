mod dfa_reduction;

use crate::automaton_graph::Automaton;
use crate::parser::Parser;

pub struct DFA {
    automaton_graph: Automaton,
}

impl DFA {
    pub fn new(automaton: Automaton) -> Self {
        DFA {
            automaton_graph: automaton,
        }
    }


}
