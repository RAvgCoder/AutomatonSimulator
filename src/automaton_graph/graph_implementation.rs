use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::automaton_graph::{
    Automaton, AutomatonType, Position, State, Symbol, Tests, Transition,
};

impl State {
    pub fn new(
        id: String,
        position: Position,
        is_accept_state: bool,
        transition_table: RefCell<Vec<Transition>>,
    ) -> State {
        State {
            id,
            position,
            is_accept_state,
            transition_edges: transition_table,
        }
    }

    /// Adds a transition to a particular state
    pub fn add_transition(&self, transition: Transition) {
        self.transition_edges.borrow_mut().push(transition)
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
}

impl fmt::Debug for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output without recursively printing the connected node
        f.debug_struct("Transition")
            .field("to", &self.to.id)
            .field(
                "symbol",
                match &self.symbol {
                    Symbol::CHAR(c) => c,
                    Symbol::EPSILON => &'ϵ',
                },
            )
            .field(
                "pop",
                match &self.pop {
                    Some(sym) => match sym {
                        Symbol::CHAR(c) => c,
                        Symbol::EPSILON => &"ϵ",
                    },
                    None => &"N/A",
                },
            )
            .field(
                "push",
                match &self.push {
                    Some(sym) => match sym {
                        Symbol::CHAR(c) => c,
                        Symbol::EPSILON => &"ϵ",
                    },
                    None => &"N/A",
                },
            )
            .finish()
    }
}

impl Automaton {
    pub fn new(
        automaton_type: AutomatonType,
        start_state: Rc<State>,
        is_in_accept_state: bool,
        accept_states: Vec<Rc<State>>,
        tests: Tests,
    ) -> Automaton {
        Automaton {
            automaton_type,
            start_state,
            is_in_accept_state,
            accept_states,
            tests,
        }
    }
}
