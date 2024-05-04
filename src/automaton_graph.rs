use std::rc::Rc;

mod graph_implementation;

/// Represents a symbol that is to be
/// transitioned on
#[derive(Debug)]
pub enum Symbol {
    CHAR(char),
    EPSILON,
}

#[derive(Debug)]
pub enum AutomatonType {
    DFA,
    NFA,
    PDA,
}

/// The transition that would be taken on the automaton
#[derive(Debug)]
pub struct Transition {
    /// Node it's connecting to
    to: Rc<Node>,
    /// Symbol to move when transitioning
    symbol: Symbol,

    /// If transition graph is a PDA

    /// Symbol pushed on the stack
    pop: Option<char>,
    /// Symbol popped from the stack
    push: Option<char>,
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub(crate) x: f64,
    pub(crate) y: f64,
}


/// Node for the graphs that represents itself,
/// and all nodes connected to it
#[derive(Debug)]
pub struct Node {
    id: String,
    position: Position,
    is_accept_state: bool,
    transition_table: Vec<Transition>,
}

/// Represents the graph of the automaton to
/// be simulated
pub struct Automaton {
    start_state: Rc<Node>,
    is_in_accept_state: bool,
    accept_states: Vec<Rc<Node>>,
}

