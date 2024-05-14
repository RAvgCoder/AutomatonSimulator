mod dfa_step_renderer;
mod equivalence_class;

use std::collections::{HashMap, HashSet};

use crate::automaton_graph::Automaton;
use crate::dfa::DFA;
use crate::dfa::dfa_reduction::dfa_step_renderer::DFAReductionStepsRenderer;
use crate::dfa::dfa_reduction::equivalence_class::EquivalenceClass;

impl DFA {
    /// Steps used by the dfa reduce function
    ///
    /// Pick a state
    ///
    /// Check if it can be divided by check transition edges if they fall into the same equivalence class
    ///
    ///----         If all states don't fall into the same equivalence class then
    ///
    ///----         Delete the current equivalence class form the list
    ///
    ///----         Then create new equivalence classes which are the subdivisions of the current equivalence class
    ///
    /// Pick another state
    pub fn reduce(&self) {
        if !self.check_if_can_reduce() {
            return;
        }

        let class_state_id = "s1";
        let connecting_points = "C1 C1";
        let padding_size = 5;

        let formatted = format!(
            "|{: <padding$}| {}",
            class_state_id,
            connecting_points,
            padding = padding_size
        );
        println!("{}", formatted);
        let class_state_id = "start";
        let formatted = format!(
            "|{: <padding$}| {}",
            class_state_id,
            connecting_points,
            padding = padding_size
        );
        println!("{}", formatted);

        // Map of sate name to transition names
        let state_map: HashMap<String, Vec<String>> =
            Automaton::get_state_map(&self.automaton_graph);
        // Break up into final and non-final states
        let mut equiv_class_list: Vec<EquivalenceClass> =
            Self::get_final_and_non_final_states(&self.automaton_graph);

        let mut reduction_steps = DFAReductionStepsRenderer::new(
            &state_map,
            &equiv_class_list,
            self.automaton_graph.all_states(),
        );

        let mut idx = 0;
        while idx < equiv_class_list.len() {
            let equiv_class = equiv_class_list.get(idx).unwrap();

            // <ClassNameConcat,Vec<State_belonging_to_the_class>>
            let mut sub_divisions: HashMap<String, Vec<String>> = HashMap::new();

            // Get name of the state dividing
            reduction_steps.add_dividing_name(equiv_class);

            for state_id in equiv_class.state_ids() {
                // Ex: [C0, C3, C4]
                let class_transition_names =
                    Self::find_class_for_transitions(&state_map, &equiv_class_list, state_id);

                reduction_steps.add_divisions(state_id, &class_transition_names);

                // Add to map
                sub_divisions
                    .entry(class_transition_names.join("")) // Use the class_name name as the hashmap key
                    .or_insert_with(Vec::new) // If the entry doesn't exist, insert a new vector
                    .push(state_id.clone()); // Add the class_state_id to the vector
            }

            // It needs to be broken up
            if sub_divisions.len() > 1 {

                // Create new subdivided classes
                let mut new_split_classes = sub_divisions
                    .values()
                    .map(|division| {
                        EquivalenceClass::new(
                            division
                                .iter()
                                .map(|string| string.clone())
                                .collect::<HashSet<String>>(),
                        )
                    })
                    .collect::<Vec<EquivalenceClass>>();

                // Remove the class to split
                equiv_class_list.swap_remove(idx);

                // Len of the equivalent class list after removal of the subdivided class
                let class_len = equiv_class_list.len();

                // Add subdivided classes
                equiv_class_list.append(&mut new_split_classes);

                // Requires that the overloaded class be removed from the list before adding new ones subdivided classes
                reduction_steps.add_split_classes(
                    &equiv_class_list[class_len..],
                    &equiv_class_list,
                );

                // Rebuild all equivalence classes
                idx = 0;
            } else {
                idx += 1;
            }

            reduction_steps.split_conclusion();
        }

        dbg!(&equiv_class_list);
        println!("{}", reduction_steps);
    }

    fn check_if_can_reduce(&self) -> bool {
        return if self.automaton_graph.all_states().is_empty() {
            println!("Cannot reduce the dfa as there are no states to reduce");
            false
        } else if self.automaton_graph.all_states()[0]
            .get_transitions()
            .is_empty()
        {
            println!("There are no transitions on this graph so it can not be reduced");
            false
        } else {
            true
        };
    }


    // Ex: [C0, C3, C4]
    /// WARNING: `equiv_states_list` must contain a list of all equivalence classes
    fn find_class_for_transitions(
        state_map: &HashMap<String, Vec<String>>,
        equiv_states_list: &Vec<EquivalenceClass>,
        class_state_id: &String,
    ) -> Vec<String> {
        state_map
            .get(class_state_id)
            .expect(&format!(
                "Could not find state {} when performing subdivisions",
                class_state_id
            ))
            .iter()
            .map(|state_name| Self::find_equiv_class_name(state_name, equiv_states_list))
            .collect::<Vec<String>>()
    }

    /// returns a list of final and non_final equivalence_classes
    /// respectively if they exist
    fn get_final_and_non_final_states(automaton: &Automaton) -> Vec<EquivalenceClass> {
        let mut final_states = HashSet::new();
        let mut non_final_states = HashSet::new();

        for state in automaton.all_states() {
            if state.is_accept_state {
                final_states.insert(state.id.clone());
            } else {
                non_final_states.insert(state.id.clone());
            }
        }

        let mut equiv_classes: Vec<EquivalenceClass> = vec![];

        // Order matters
        let final_state_index = 0;
        let mut non_final_state_index = 1;
        let mut splits = vec![
            EquivalenceClass::new(final_states),
            EquivalenceClass::new(non_final_states),
        ];

        if !splits[final_state_index].is_empty() {
            equiv_classes.push(splits.remove(final_state_index));
            non_final_state_index = 0;
        }

        if !splits[non_final_state_index].is_empty() {
            equiv_classes.push(splits.remove(non_final_state_index))
        }

        equiv_classes
    }
    fn find_equiv_class_name(
        state_name: &String,
        equiv_states_list: &Vec<EquivalenceClass>,
    ) -> String {
        equiv_states_list
            .iter()
            .find(|&equiv_class| equiv_class.state_ids().contains(state_name))
            .expect(&format!(
                "Could not find {} in any of the equivalence classes",
                state_name
            ))
            .class_name()
            .clone()
    }
}