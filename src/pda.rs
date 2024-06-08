use std::rc::Rc;

use crate::automaton_graph::{Automaton, Symbol};

mod pda_simulator;

pub struct PDA {
    automaton_graph: Rc<Automaton>,
}

impl PDA {
    pub fn new(automaton: Rc<Automaton>) -> PDA {
        PDA {
            automaton_graph: automaton,
        }
    }
}
