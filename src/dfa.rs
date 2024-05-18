use crate::automaton_graph::Automaton;

mod dfa_reduction;

pub struct DFA {
    automaton_graph: Automaton,
}

#[derive(Debug)]
pub struct ReductionSteps {
    classes_created: u32,
    pub steps: Vec<String>,
    pub table: Vec<String>,
    pub reduced_dfa: Automaton,
}

impl ReductionSteps {
    fn classes_created(&self) -> u32 {
        self.classes_created
    }
}

impl DFA {
    pub fn new(automaton: Automaton) -> Self {
        // Makes all transition have the same order of symbols
        for s in automaton.all_states() {
            s.transition_edges.borrow_mut().sort()
        }

        DFA {
            automaton_graph: automaton,
        }
    }
}
