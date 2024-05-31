use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::automaton_graph::{Symbol, Transition};
use crate::dfa::DFA;

pub enum SimulationError {
    NonTransitionSymbols(String),
}

impl Debug for SimulationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SimulationError::NonTransitionSymbols(msg) => {
                write!(f, "Non-transition symbols error: {}", msg)
            }
        }
    }
}

impl DFA {
    pub fn simulate(&self, simulating_string: &String) -> Result<bool, SimulationError> {
        let mut curr_state = self.automaton_graph.get_start_state();

        for (idx, c) in simulating_string.chars().enumerate() {
            let transition = Transition::find_transition_by_symbol(curr_state.get_transitions(), c)
                .ok_or_else(|| SimulationError::NonTransitionSymbols(format!(
                    "Invalid character {} at index {} for symbols {:?} on state {}",
                    c,
                    idx,
                    curr_state
                        .transition_edges
                        .borrow()
                        .iter()
                        .map(|t| t.transition_on())
                        .collect::<Vec<Symbol>>(),
                    &curr_state.alt_id
                )))?;

            curr_state = transition.to();
        }

        Ok(curr_state.is_accept_state)
    }
}
