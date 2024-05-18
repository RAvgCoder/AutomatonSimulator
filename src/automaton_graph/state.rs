use crate::automaton_graph::{Position, State, Transition};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

impl State {
    pub fn new(
        id: String,
        alt_id: Option<String>,
        position: Position,
        is_accept_state: bool,
        transition_table: RefCell<Vec<Transition>>,
    ) -> State {
        State {
            id: id.clone(),
            alt_id: alt_id.unwrap_or(id),
            position,
            is_accept_state,
            transition_edges: transition_table,
        }
    }

    /// Adds a transition to a particular state
    pub fn add_transition(&self, transition: Transition) {
        self.transition_edges.borrow_mut().push(transition)
    }

    /// Returns an immutable reference to the list of transitions from the current state
    pub fn get_transitions(&self) -> Ref<Vec<Transition>> {
        self.transition_edges.borrow()
    }

    /// Define a function to search for a node by its ID
    pub fn find_state_by_id(states: &Vec<Rc<State>>, target_id: &str) -> Option<Rc<State>> {
        states.iter().find(|node| node.id == target_id).cloned()
    }
}
