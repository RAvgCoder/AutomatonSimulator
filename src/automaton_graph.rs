use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

mod automaton;
mod state;
mod symbol;
mod transition;

/// Represents a symbol that is to be
/// transitioned on
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Symbol {
    CHAR(char),
    EPSILON, // ϵ
}

/// Represents an automaton type
#[derive(Debug, Copy, Clone)]
pub enum AutomatonType {
    DFA,
    NFA,
    PDA,
}

/// The transition that would be taken on the automaton
pub struct Transition {
    /// State it's connecting to
    to: Rc<State>,
    /// Symbol to move when transitioning
    symbol: Symbol,

    /// If transition graph is a PDA

    /// Symbol pushed on the stack
    pop_symbol: Option<Symbol>,
    /// Symbol popped from the stack
    push_symbol: Option<Symbol>,
}

/// Position on the screen to be rendered
#[derive(Copy, Clone, Debug, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Node for the graphs that represents itself,
/// and all nodes connected to it
#[derive(Debug)]
pub struct State {
    pub id: String,
    pub alt_id: String,
    pub position: Position,
    pub is_accept_state: bool,
    /// The transitions that can be taken form this state
    pub transition_edges: RefCell<Vec<Transition>>,
}

/// Represents the graph of the automaton to
/// be simulated
#[derive(Debug)]
pub struct Automaton {
    pub automaton_type: AutomatonType,
    start_state: Rc<State>,
    is_in_accept_state: bool,
    accept_states: Vec<Rc<State>>,
    all_states: Vec<Rc<State>>,
    pub tests: Tests,
}

/// Represents a test suite for strings to be accepted
/// or rejected by the automaton
#[derive(Default, Debug)]
pub struct Tests {
    pub accepting_strings: Vec<String>,
    pub rejecting_strings: Vec<String>,
}

pub enum SimulationError {
    // PDAs
    NoTransitionForSymbol(String),
    MultipleTransitionsFound(String),
    IllegalPopFromStack(String),
}

impl Debug for SimulationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PDA Simulation Err: ")?;
        match self {
            SimulationError::NoTransitionForSymbol(msg) => {
                write!(f, "NoTransitionForSymbol: {}", msg)
            }
            SimulationError::MultipleTransitionsFound(msg) => {
                write!(f, "MultipleTransitionsFound: {}", msg)
            }
            SimulationError::IllegalPopFromStack(msg) => {
                write!(f, "IllegalPopFromStack: {}", msg)
            }
        }
    }
}
