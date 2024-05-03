/// Represents a symbol that is to be
/// transitioned on
enum Symbol {
    CHAR(char),
    EPSILON,
}

/// The State the automaton is in
pub enum State {
    ACCEPT,
    REJECT,
}

#[derive(Debug)]
pub enum AutomatonType {
    DFA,
    NFA,
    PDA,
}

/// The transition that would be taken on the automaton
struct Transition {
    /// Node it's connecting to
    to: Node,
    /// Symbol to move when transitioning
    symbol: Symbol,

    /// If transition graph is a PDA

    /// Symbol pushed on the stack
    pop: Option<char>,
    /// Symbol popped from the stack
    push: Option<char>,
}

struct Position {
    x: usize,
    y: usize,
}

/// Node for the graphs that represents itself,
/// and all nodes connected to it
struct Node {
    id: String,
    position: Position,
    transition_table: Vec<Transition>,
}

/// Represents the graph of the automaton to
/// be simulated
pub struct Automaton<'a> {
    start_state: Node,
    current_state: State,
    accept_states: Vec<&'a Node>,
}

impl Transition {}
