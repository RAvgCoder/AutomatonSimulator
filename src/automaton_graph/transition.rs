use crate::automaton_graph::{State, Symbol, Transition};
use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

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
            pop: None,
            push: None,
        }
    }

    /// Ret
    pub fn to(&self) -> Rc<State> {
        self.to.clone()
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
