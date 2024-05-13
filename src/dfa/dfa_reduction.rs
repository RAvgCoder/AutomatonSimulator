use std::collections::{HashMap, HashSet};

use crate::automaton_graph::Automaton;
use crate::dfa::dfa_reduction::equivalence_class::EquivalenceClass;
use crate::dfa::DFA;

impl DFA {
    pub fn reduce(&self) {
        // Map of sate name to transition names
        let state_map: HashMap<String, Vec<String>> =
            Automaton::get_state_map(&self.automaton_graph);
        // Break up into final and non-final states
        let mut equiv_states_list: Vec<EquivalenceClass> =
            Self::get_final_and_non_final_states(&self.automaton_graph);

        let mut idx = 0;
        while idx < equiv_states_list.len() {
            let equiv_state = equiv_states_list.get(idx).unwrap();
            // <ClassNameConcat,Vec<State_belonging_to_the_class>>
            let mut sub_divisions: HashMap<String, Vec<String>> = HashMap::new();

            for class_state_id in equiv_state.class_state_ids() {
                let class_name = state_map
                    .get(class_state_id)
                    .expect(&format!(
                        "Could not find state {} when performing subdivisions",
                        class_state_id
                    ))
                    .iter()
                    .fold(String::new(), |mut acc, state_name| {
                        acc.push_str(
                            Self::find_equiv_class_name(state_name, &equiv_states_list).as_str(),
                        );
                        acc
                    });

                // Add to map
                sub_divisions
                    .entry(class_name) // Use the class_name name as the hashmap key
                    // If the entry doesn't exist, insert a new vector
                    .or_insert_with(Vec::new)
                    .push(class_state_id.clone()); // Add the class_state_id to the vector
            }

            // It needs to be broken up
            if sub_divisions.len() > 1 {
                dbg!(&equiv_states_list);
                println!();

                // Add subdivided classes
                equiv_states_list.swap_remove(idx);

                // equiv_states_list.extend(sub_divisions.values().map(|list| {
                //     EquivalenceClass::new(
                //         list.iter()
                //             .map(|string| string.clone())
                //             .collect::<HashSet<String>>()
                //     )
                // }));

                for list in sub_divisions.values() {
                    equiv_states_list.push(EquivalenceClass::new(
                        list.iter()
                            .map(|string| string.clone())
                            .collect::<HashSet<String>>(),
                    ))
                }

                // Rebuild all equivalence classes
                idx = 0;
            } else {
                idx += 1
            }
        }

        dbg!(&equiv_states_list);
    }

    /// returns a list of final and non_final equivalence_classes
    /// respectively if they exist
    fn get_final_and_non_final_states(automaton: &Automaton) -> Vec<EquivalenceClass> {
        let mut final_states = HashSet::new();
        let mut non_final_states = HashSet::new();

        for state in automaton.state_list() {
            let state = state.clone();
            if state.is_accept_state {
                final_states.insert(state.id.clone());
            } else {
                non_final_states.insert(state.id.clone());
            }
        }

        let mut equiv_classes = vec![];

        if !final_states.is_empty() {
            equiv_classes.push(EquivalenceClass::new(final_states))
        }

        if !non_final_states.is_empty() {
            equiv_classes.push(EquivalenceClass::new(non_final_states))
        }

        equiv_classes
    }
    fn find_equiv_class_name(
        state_name: &String,
        equiv_states_list: &Vec<EquivalenceClass>,
    ) -> String {
        equiv_states_list
            .iter()
            .find(|&equiv_class| equiv_class.class_state_ids().contains(state_name))
            .expect(&format!(
                "Could not find {} in any of the equivalence classes",
                state_name
            ))
            .class_name()
            .clone()
    }
}

mod equivalence_class {
    use std::collections::HashSet;

    static mut CLASS_COUNTER: u32 = 0;

    #[derive(Debug)]
    pub struct EquivalenceClass {
        class_name: String,
        class_state_ids: HashSet<String>,
    }

    impl EquivalenceClass {
        // Static counter to keep track of the next available class number

        pub(in crate::dfa::dfa_reduction) fn new(
            class_state_ids: HashSet<String>,
        ) -> EquivalenceClass {
            // Increment the class counter
            unsafe {
                let class_number = Self::get_next_class_number();
                let class_name = format!("C{}", class_number);
                EquivalenceClass {
                    class_name,
                    class_state_ids,
                }
            }
        }

        pub(in crate::dfa::dfa_reduction) fn class_state_ids(&self) -> &HashSet<String> {
            &self.class_state_ids
        }

        pub(in crate::dfa::dfa_reduction) fn class_name(&self) -> &String {
            &self.class_name
        }

        pub(in crate::dfa::dfa_reduction) fn add_to_class_state(&mut self, state_ids: String) {
            self.class_state_ids.insert(state_ids);
        }

        // Helper method to get the next available class number
        unsafe fn get_next_class_number() -> u32 {
            let class_number = CLASS_COUNTER;
            CLASS_COUNTER += 1;
            class_number
        }
    }
}
