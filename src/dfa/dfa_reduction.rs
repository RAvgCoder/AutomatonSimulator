use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::automaton_graph::{
    Automaton, AutomatonType, Position, State, Symbol, Tests, Transition,
};
use crate::dfa::dfa_reduction::dfa_step_renderer::DFAReductionStepsRenderer;
use crate::dfa::dfa_reduction::equivalence_class::EquivalenceClass;
use crate::dfa::{ReductionSteps, DFA};

mod dfa_step_renderer;
mod equivalence_class;

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
    pub fn reduce(&self) -> Option<ReductionSteps> {
        // Map of sate name to transition names
        let state_map: HashMap<String, Vec<String>> =
            Automaton::get_state_map(&self.automaton_graph);
        // Break up into final and non-final states
        let mut equiv_class_list: Vec<EquivalenceClass> =
            Self::get_final_and_non_final_states(&self.automaton_graph);

        // Check if a reduction is possible
        if !self.check_if_can_reduce(&equiv_class_list) {
            return None;
        }

        // Create a step renderer
        let mut step_renderer: DFAReductionStepsRenderer = DFAReductionStepsRenderer::new(
            &state_map,
            &equiv_class_list,
            self.automaton_graph
                .all_states()
                .first()
                .expect("Cannot reduce dfa with no states"),
        );

        // Reduce the dfa
        let mut idx = 0;
        while idx < equiv_class_list.len() {
            // Class picked for subdivision
            let equiv_class: &EquivalenceClass = equiv_class_list.get(idx).unwrap();

            // <ClassNameConcat,Vec<State_belonging_to_the_class>>
            let mut sub_divisions: HashMap<String, Vec<String>> = HashMap::new();

            // Add the name of the state were are to possibly divide
            step_renderer.add_name_of_sub_dividing_class(equiv_class);

            // Check if state should be subdivided
            for state_id in equiv_class.state_ids() {
                // Ex: [C0, C3, C4]
                let class_transition_names =
                    Self::find_class_for_transitions(&state_map, &equiv_class_list, state_id);

                step_renderer.track_sub_divisions(state_id, &class_transition_names);

                // Collect all class_transitions to a map so see if they are all unique
                // This also acts like a group by function grouping by class_transition names
                // mapping to list of states which have similar transition names
                sub_divisions
                    .entry(class_transition_names) // Use the class_name name as the hashmap key
                    .or_insert_with(Vec::new) // If the entry doesn't exist, insert a new vector
                    .push(state_id.clone()); // Add the class_state_id to the vector
            }

            // Class needs to be broken up as not all states map to the same
            // equivalent class
            if sub_divisions.len() > 1 {
                // Creates new subdivided classes
                let mut new_divided_classes = sub_divisions
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
                equiv_class_list.append(&mut new_divided_classes);

                // Requires that the class that's to be divided be removed from the list before
                // adding the new ones subdivided classes
                step_renderer.add_split_classes(&equiv_class_list[class_len..], &equiv_class_list);

                // Rebuild all equivalence classes
                idx = 0;
            } else {
                idx += 1;
            }

            // Add remarks on the state of the subdivisions
            step_renderer.split_conclusion();
        }
        step_renderer.finish(&equiv_class_list);
        
        Some(ReductionSteps {
            classes_created: equiv_class_list.len() as u32,
            table: step_renderer.move_table_steps(),
            steps: step_renderer.move_steps(),
            reduced_dfa: Self::class_to_automaton(
                equiv_class_list,
                step_renderer.transitions_alphabets(),
                self.automaton_graph.all_states(),
                &state_map,
            ),
        })
    }

    /// Acts like an assert to check if the automaton qualifies for a reduction
    fn check_if_can_reduce(&self, equiv_class_list: &Vec<EquivalenceClass>) -> bool {
        return if self.automaton_graph.all_states().is_empty() {
            eprintln!("Cannot reduce the dfa as there are no states to reduce");
            false
        } else if self.automaton_graph.all_states()[0]
            .get_transitions()
            .is_empty()
        {
            eprintln!("There are no transitions on this graph so it can not be reduced");
            false
        } else if equiv_class_list.len() != 2 {
            eprintln!(
                "It only contains either final / non-final sates hence no need for a reduction"
            );
            false
        } else {
            true
        };
    }

    /// Returns a string of classes corresponding to the transitions on a `class_state_id`
    ///
    /// Ex: "C0 C3 C4"
    ///
    /// # Arguments
    ///
    /// * `state_map`: A map of all states and a list of states they transition to
    /// * `complete_equiv_states_list`: This `must` contain a list of all equivalence classes
    /// * `class_state_id`: The state you would like to find the equivalent class for each transitions
    fn find_class_for_transitions(
        state_map: &HashMap<String, Vec<String>>,
        complete_equiv_states_list: &Vec<EquivalenceClass>,
        class_state_id: &str,
    ) -> String {
        state_map
            .get(class_state_id)
            .expect(&format!(
                "Could not find state {} when performing subdivisions",
                class_state_id
            ))
            .iter()
            .fold(String::new(), |mut acc, state_name| {
                acc.push_str(&EquivalenceClass::find_equiv_class_name(
                    state_name,
                    complete_equiv_states_list,
                ));
                acc.push(' ');
                acc
            })
    }

    /// Returns a list of final and non_final equivalence_classes
    /// respectively if they exist
    ///
    /// # Arguments
    ///
    /// * `automaton`: The automaton to reduce
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

        if !final_states.is_empty() {
            equiv_classes.push(EquivalenceClass::new(final_states));
        }

        if !non_final_states.is_empty() {
            equiv_classes.push(EquivalenceClass::new(non_final_states))
        }

        equiv_classes
    }

    fn class_to_automaton(
        equivalence_classes: Vec<EquivalenceClass>,
        transitions_alphabets: &Vec<Symbol>,
        all_states: &Vec<Rc<State>>,
        state_map: &HashMap<String, Vec<String>>,
    ) -> Automaton {
        // Create list of states without transitions
        let new_states = equivalence_classes
            .iter()
            .map(|eq| {
                Rc::new(State::new(
                    eq.name(),
                    None,
                    Position::default(),
                    // Check if this class contains a final state
                    eq.state_ids()
                        .iter()
                        .find(|ids| {
                            State::find_state_by_id(all_states, ids)
                                .expect("Could not find id in the list of states")
                                .is_accept_state
                        })
                        .map_or(false, |_| true),
                    RefCell::new(vec![]),
                ))
            })
            .collect::<Vec<Rc<State>>>();

        let err_message = "equivalent_class id not found even after creating new states from equivalent_class list";

        // Add transitions for each state
        for eq in &equivalence_classes {
            let curr_state = State::find_state_by_id(&new_states, &eq.name()).expect(err_message);

            // Find any state in the equivalence list
            let state = eq
                .state_ids()
                .iter()
                .next()
                .expect("An equivalent class should never have empty state ids");

            // Get that states connections and add its new transitions to the curr_state
            state_map
                .get(state)
                .expect("Could not find state in map")
                .iter()
                .map(|c| {
                    let class_name =
                        EquivalenceClass::find_equiv_class_name(c, &equivalence_classes);
                    State::find_state_by_id(&new_states, &class_name).expect(err_message)
                })
                .zip(transitions_alphabets)
                .for_each(|(state, symbol): (Rc<State>, &Symbol)| {
                    curr_state.add_transition(Transition::dfa(state, *symbol))
                })
        }

        let s_state = State::find_state_by_id(
            &new_states,
            &EquivalenceClass::find_equiv_class_name("start", &equivalence_classes),
        )
        .expect("Cannot find start state when recreating the automaton");

        let accepting_states = new_states
            .iter()
            .filter_map(|s| {
                if s.is_accept_state {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Rc<State>>>();

        Automaton::new(
            AutomatonType::DFA,
            s_state,
            accepting_states,
            new_states,
            Tests::default(),
        )
    }
}
