use std::cell::Ref;
use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use crate::automaton_graph::{State, Symbol, Transition};

impl Transition {
    /// Creates a full [Transition] object with options to add pop and push options
    /// compared to [Transition::dfa] which only creates a transition for strictly DFAs
    ///
    /// # Arguments
    ///
    /// * `to`: The state you are transitioning to
    /// * `symbol`: Symbol your transitioning on
    /// * `pop`: Symbol to pop from the stack on a transition
    /// * `push`: Symbol to push from the stack on a transition
    pub fn new(
        to: Rc<State>,
        symbol: Symbol,
        pop_symbol: Option<Symbol>,
        push_symbol: Option<Symbol>,
    ) -> Transition {
        Transition {
            to,
            symbol,
            pop_symbol,
            push_symbol,
        }
    }

    /// Creates a DFA [Transition] without pop and push parameters
    ///
    /// # Arguments
    ///
    /// * `to`: State you are transitioning to
    /// * `symbol`: Symbol to transition on
    pub fn dfa(to: Rc<State>, symbol: Symbol) -> Transition {
        Transition {
            to,
            symbol,
            pop_symbol: None,
            push_symbol: None,
        }
    }

    /// Returns the id of the State pointing to
    pub fn next_state_id(&self) -> &String {
        &self.to.id
    }

    /// Returns a strong reference to the [State] its pointing to
    pub fn to(&self) -> Rc<State> {
        self.to.clone()
    }

    pub fn transition_on(&self) -> Symbol {
        self.symbol
    }

    /// Value to be pushed to the stack
    pub fn push_symbol(&self) -> Option<Symbol> {
        self.push_symbol
    }

    /// Value to be popped from the stack
    pub fn pop_symbol(&self) -> Option<Symbol> {
        self.pop_symbol
    }

    /// Finds all transitions that can be taken if given
    /// a particular symbol
    ///
    /// # Arguments
    ///
    /// * `transitions`: List of transitions
    /// * `symbol`: The char equivalent of that symbol
    /// Note to check for transitions on epsilon use [Self::find_epsilon_transitions]
    pub fn find_transition_by_symbol(
        transitions: Ref<Vec<Transition>>,
        symbol: char,
    ) -> Vec<Transition> {
        transitions
            .iter()
            .filter_map(|transition| {
                if char::from(transition.symbol) == symbol {
                    Some(transition.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Transition>>()
    }

    /// Finds all transitions that can be taken on an
    /// epsilon transition [Symbol::EPSILON] \ ϵ
    ///
    /// # Arguments
    ///
    /// * `transitions`: List of transitions
    pub fn find_epsilon_transitions(transitions: Ref<Vec<Transition>>) -> Vec<Transition> {
        Transition::find_transition_by_symbol(transitions, char::from(Symbol::EPSILON))
    }
}

impl Clone for Transition {
    fn clone(&self) -> Self {
        Transition {
            to: Rc::clone(&self.to),
            symbol: self.symbol,
            pop_symbol: self.pop_symbol,
            push_symbol: self.push_symbol,
        }
    }
}

impl fmt::Debug for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output without recursively printing the connected node
        f.debug_struct("Transition")
            .field("to", &self.to.id)
            .field("symbol", &self.symbol)
            .field("pop_symbol", &self.pop_symbol)
            .field("push_symbol", &self.push_symbol)
            .finish()
    }
}

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Eq for Transition {}

impl PartialOrd for Transition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Transition {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.symbol, &other.symbol) {
            (Symbol::CHAR(c1), Symbol::CHAR(c2)) => c1.cmp(c2),
            // For other cases, consider them equal
            _ => Ordering::Equal,
        }
    }
}
