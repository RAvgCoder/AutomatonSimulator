use std::fmt::{Display, Formatter};

use crate::automaton_graph::Automaton;
use crate::dfa::dfa_reduction::{Steps, Table};

mod dfa_reduction;

pub struct DFA {
    automaton_graph: Automaton,
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

#[derive(Debug)]
pub struct ReductionSteps {
    num_of_classes_created: u32,
    pub steps: Steps,
    pub table: Table,
    pub reduced_dfa: Automaton,
}

impl ReductionSteps {
    fn num_of_classes_created(&self) -> u32 {
        self.num_of_classes_created
    }
}

impl Display for ReductionSteps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Number of equivalence classes created: {}",
            self.num_of_classes_created
        )?;
        writeln!(f, "{}", self.steps)?;
        writeln!(f, "{}", self.table)?;
        writeln!(f, "{:#?}", self.reduced_dfa)?;
        Ok(())
    }
}
