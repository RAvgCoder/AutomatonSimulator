use std::collections::HashMap;
use std::rc::Rc;

use crate::automaton_graph::State;
use crate::dfa::DFA;

struct EquivalenceClass {
    cass_name: String,
    class_states: Vec<Rc<State>>,
}

impl DFA {
    pub fn reduce(&self) {
        let (state_map, state_vec) = self.get_state_map();
        
        
    }


    fn get_state_map(&self) -> (HashMap<String, &Rc<State>>, &Vec<Rc<State>>)
    {
        let state_vec = self.automaton_graph.state_list();
        let state_map: HashMap<String, &Rc<State>> = state_vec
            .iter()
            .fold(
                HashMap::new(),
                |mut hash_map, state| {
                    hash_map.insert(state.id.clone(), state);
                    hash_map
                });

        (state_map, state_vec)
    }
}
