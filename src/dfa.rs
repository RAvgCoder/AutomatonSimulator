use crate::automaton_graph::Automaton;

mod dfa_reduction;

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
