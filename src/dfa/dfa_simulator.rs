use std::fmt::Debug;

use crate::automaton_graph::{SimulationError, Symbol, Transition};
use crate::dfa::DFA;

impl DFA {
    pub fn simulate(&self, simulating_string: &str) -> Result<bool, SimulationError> {
        let mut curr_state = self.automaton_graph.get_start_state();

        for (idx, c) in simulating_string.chars().enumerate() {
            let transition = Transition::find_transition_by_symbol(curr_state.get_transitions(), c);

            if transition.len() != 1 {
                // DFAs must only have one valid transition
                return Err(SimulationError::NoTransitionForSymbol(format!(
                    "{} found for character {} at index {} for symbols {:?} on state {}",
                    if transition.len() == 0 {
                        "No transition"
                    } else {
                        "Multiple transitions"
                    },
                    c,
                    idx,
                    curr_state
                        .transition_edges
                        .borrow()
                        .iter()
                        .map(|t| t.transition_on())
                        .collect::<Vec<Symbol>>(),
                    &curr_state.alt_id
                )));
            }

            curr_state = transition.first().unwrap().to();
        }

        Ok(curr_state.is_accept_state)
    }
}
