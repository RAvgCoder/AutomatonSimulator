use std::collections::HashMap;
use std::rc::{Rc, Weak};

use crate::automaton_graph::{Automaton, AutomatonType, State, Tests};

impl Automaton {
    pub fn new(
        automaton_type: AutomatonType,
        start_state: Rc<State>,
        accept_states: Vec<Rc<State>>,
        all_states: Vec<Rc<State>>,
        tests: Tests,
    ) -> Automaton {
        Automaton {
            automaton_type,
            start_state,
            is_in_accept_state: false,
            accept_states,
            all_states,
            tests,
        }
    }

    pub fn get_start_state(&self) -> Rc<State> {
        self.start_state.clone()
    }

    pub fn all_states(&self) -> &Vec<Rc<State>> {
        &self.all_states
    }

    /// Returns a hashmap which contains
    pub fn get_state_map(automaton: &Automaton) -> HashMap<String, Vec<String>> {
        automaton
            .all_states
            .iter()
            .fold(HashMap::new(), |mut hash_map, state| {
                hash_map.insert(
                    state.id.clone(),
                    // Create a vec of all transition ids from state
                    state
                        .transition_edges
                        .borrow()
                        .iter()
                        .map(|transition| transition.next_state_id().clone())
                        .collect::<Vec<String>>(),
                );
                hash_map
            })
    }
}
