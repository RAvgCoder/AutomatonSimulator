use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::automaton_graph::{
    Automaton, AutomatonType, Position, State, Symbol, Tests, Transition,
};

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Symbol::CHAR(c) => write!(f, "{}", c),
            Symbol::EPSILON => write!(f, "ϵ"),
        }
    }
}

impl State {
    pub fn new(
        id: String,
        alt_id: String,
        position: Position,
        is_accept_state: bool,
        transition_table: RefCell<Vec<Transition>>,
    ) -> State {
        State {
            id,
            alt_id,
            position,
            is_accept_state,
            transition_edges: transition_table,
        }
    }

    /// Adds a transition to a particular state
    pub fn add_transition(&self, transition: Transition) {
        self.transition_edges.borrow_mut().push(transition)
    }

    pub fn get_transitions(&self) -> Ref<Vec<Transition>> {
        self.transition_edges.borrow()
    }
}

impl Transition {
    pub fn new(
        to: Rc<State>,
        symbol: Symbol,
        pop: Option<Symbol>,
        push: Option<Symbol>,
    ) -> Transition {
        Transition {
            to,
            symbol,
            pop,
            push,
        }
    }

    pub fn to(&self) -> &Rc<State> {
        &self.to
    }

    pub fn transition_on(&self) -> Symbol {
        self.symbol
    }
}

impl fmt::Debug for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output without recursively printing the connected node
        f.debug_struct("Transition")
            .field("to", &self.to.id)
            .field("symbol", &self.symbol)
            .field("pop", &self.pop)
            .field("push", &self.push)
            .finish()
    }
}

impl Automaton {
    pub fn new(
        automaton_type: AutomatonType,
        start_state: Rc<State>,
        is_in_accept_state: bool,
        accept_states: Vec<Rc<State>>,
        all_states: Vec<Rc<State>>,
        tests: Tests,
    ) -> Automaton {
        Automaton {
            automaton_type,
            start_state,
            is_in_accept_state,
            accept_states,
            all_states,
            tests,
        }
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
                        .map(|transition| transition.to().id.clone())
                        .collect::<Vec<String>>(),
                );
                hash_map
            })
    }
}
