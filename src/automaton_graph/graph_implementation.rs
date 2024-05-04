use std::rc::Rc;

use crate::automaton_graph::{Node, Position, Symbol, Transition};

impl Node {
    /// Creates a new node
    pub fn new(
        id: String,
        position: Position,
        is_accept_state: bool,
        transition_table: Vec<Transition>,
    ) -> Node {
        Node {
            id,
            position,
            is_accept_state,
            transition_table,
        }
    }


    pub fn add_to_transition_table(&mut self, transition: Transition) {
        self.transition_table.push(transition)
    }
}

impl Transition {
    pub fn new(
        to: Rc<Node>,
        symbol: Symbol,
        pop: Option<char>,
        push: Option<char>
    ) -> Transition {
        Transition {
            to,
            symbol,
            pop,
            push,
        }
    }
}