use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::automaton_graph::{Automaton, State, Symbol};
use crate::dfa::DFA;
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

        let mut reduction_steps = DFAReductionSteps::new(
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
                // dbg!(&equiv_states_list);
                println!();

                let last_equiv_class_num = EquivalenceClass::get_current_num();

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
                    // [last_equiv_class_num as usize, sub_divisions.len()],
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

/// Holds steps to Displays a step-by-step solution for reducing a dfa
#[derive(Debug)]
struct DFAReductionSteps<'a> {
    table_steps: Vec<String>,
    steps: Vec<String>,
    state_transition_map: &'a HashMap<String, Vec<String>>,
    transitions_name_map: Vec<Symbol>,
    padding_size: usize,
    reduction_was_successful: bool,
    class_name_dividing: String,
}

impl<'a> DFAReductionSteps<'a> {
    pub fn new(
        state_transition_map: &'a HashMap<String, Vec<String>>,
        first_division: &Vec<EquivalenceClass>,
        all_states: &Vec<Rc<State>>,
    ) -> DFAReductionSteps<'a> {
        assert!(
            first_division.len() <= 2,
            "First division was not properly split into final and no final states"
        );

        let padding_size = 7;

        DFAReductionSteps {
            table_steps: vec![Self::create_table_as_string(
                None,
                first_division,
                state_transition_map,
                padding_size,
            )],
            steps: vec![],
            state_transition_map,
            transitions_name_map: all_states[0]
                .get_transitions()
                .iter()
                .map(|transition| transition.transition_on())
                .collect::<Vec<Symbol>>(),
            padding_size,
            reduction_was_successful: false,
            class_name_dividing: String::new(),
        }
    }

    pub fn add_split_classes(
        &mut self,
        new_split_classes: &[EquivalenceClass],
        complete_equiv_class_list: &Vec<EquivalenceClass>,
        // class_num_before_split_and_num_of_new_classes: [usize; 2],
    ) {
        // + 1 because a class was just decomposed into other classes
        // let current_class_number = class_num_before_split_and_num_of_new_classes[0] + 1;
        // let num_of_new_classes = class_num_before_split_and_num_of_new_classes[1];

        self.reduction_was_successful = true;
        self.steps.push(format!(
            "Since all states in {} dont fall into the same equivalence class it is then split into [{}]",
            self.class_name_dividing,
            new_split_classes.iter()
                .fold(String::new(), |mut acc, classes| {
                    acc.push_str(classes.class_name());
                    acc.push(',');
                    acc
                })
        ));
        self.steps.push(String::from("These include:"));
        self.steps.push(Self::create_table_as_string(
            Some(new_split_classes),
            complete_equiv_class_list,
            self.state_transition_map,
            self.padding_size,
        ));

        // Self::render_and_add_transition_and_classes_to_list(
        //     full_equivalence_classes,
        //     state_transition_map,
        //     padding_size,
        //     &mut table,
        //     equivalence_class,
        // );

        // Add to new iteration to table
        self.table_steps.push(Self::create_table_as_string(
            None,
            complete_equiv_class_list,
            self.state_transition_map,
            self.padding_size,
        ))
    }

    pub fn split_conclusion(&mut self) {
        self.steps.push(if !self.reduction_was_successful {
            format!(
                "Could not split {} as all state transitions fall into the same equivalent classes", self.class_name_dividing
            )
        } else {
            format!("Since splitting {} was successful we have to re-evaluate all classes again to see if any have changed", self.class_name_dividing)
        })
    }

    pub fn add_dividing_name(&mut self, class_dividing: &EquivalenceClass) {
        self.steps.push(format!(
            "We now pick and try to subdivide the equivalence classes {}",
            class_dividing.class_name()
        ));

        // Reset the reduction checking
        self.reduction_was_successful = false;
        self.class_name_dividing = class_dividing.class_name().clone()
    }

    pub fn add_divisions(&mut self, state_id: &String, class_transition_names: &Vec<String>) {
        self.steps.push(format!(
            "{: <padding$}|   {}",
            state_id,
            class_transition_names.join(" "),
            padding = self.padding_size
        ));
    }
    fn create_table_as_string(
        partial_equivalence_classes: Option<&[EquivalenceClass]>,
        full_equivalence_classes: &Vec<EquivalenceClass>,
        state_transition_map: &HashMap<String, Vec<String>>,
        padding_size: usize,
    ) -> String {
        let mut table = String::new();

        for equivalence_class in partial_equivalence_classes.unwrap_or(full_equivalence_classes) {
            Self::render_class_name(&mut table, equivalence_class, padding_size);

            Self::render_and_add_transition_and_classes_to_list(
                full_equivalence_classes,
                state_transition_map,
                padding_size,
                &mut table,
                equivalence_class,
            );

            table.push('\n');
        }
        // dbg!(&table);

        // Return the table created
        table
    }

    /// Render a class name in this format    
    /// |--------------------
    ///
    /// |        ClassName
    ///
    /// |--------------------
    fn render_class_name(
        storage: &mut String,
        equivalence_class: &EquivalenceClass,
        padding_size: usize,
    ) {
        let spacing = "-".repeat(padding_size * 2);
        storage.push_str(&format!("|{}", spacing));
        storage.push('\n');
        storage.push_str(&format!(
            "|{name: >padding$}",
            name = equivalence_class.class_name(),
            padding = padding_size
        ));
        storage.push('\n');
        storage.push_str(&format!("|{}", spacing));
        storage.push('\n');
    }

    /// Renders a list of transition
    ///  
    /// |start| C0 C0
    ///
    ///  |s3  | C0 C0
    ///
    ///  |s1  | C1 C1
    ///
    ///  |s0  | C1 C1
    ///
    /// WARNING: `equivalence_classes` must contain a list of all equivalence classes
    fn render_and_add_transition_and_classes_to_list(
        equivalence_classes: &Vec<EquivalenceClass>,
        state_transition_map: &HashMap<String, Vec<String>>,
        padding_size: usize,
        storage: &mut String,
        equivalence_class: &EquivalenceClass,
    ) {
        for class_state_id in equivalence_class.state_ids() {
            let connecting_points = DFA::find_class_for_transitions(
                state_transition_map,
                equivalence_classes,
                class_state_id,
            )
                .join(" ");

            storage.push_str(&format!(
                "|{: <padding$}| {}",
                class_state_id,
                connecting_points,
                padding = padding_size
            ));
            storage.push('\n');
        }
    }
}

impl<'a> Display for DFAReductionSteps<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Steps for reducing the dfa are as follows:")?;
        for step in &self.steps {
            writeln!(f, "{}", step)?;
        }

        writeln!(f, "\n")?;
        write!(f, "Class names are shown in order of connection names: [")?;
        for (idx, symbol) in self.transitions_name_map.iter().enumerate() {
            write!(f, "{}", symbol)?;
            if !(idx == self.transitions_name_map.len() - 1) {
                write!(f, ",")?;
            }
        }
        write!(f, "]\n")?;
        writeln!(f, "Tables generated from reduction are")?;
        let spacing = "-".repeat(self.padding_size * 4);
        for (iter_count, table) in self.table_steps.iter().enumerate() {
            writeln!(f, "{}", spacing)?;
            writeln!(f, "Iteration count {}:\n{}", iter_count + 1, table)?;
        }
        writeln!(f, "{}", spacing)?;

        Ok(())
    }
}

mod equivalence_class {
    use std::collections::HashSet;

    const NAME: char = 'C';
    const START_COUNT: u8 = 0;
    static mut CLASS_COUNTER: u32 = START_COUNT as u32;

    #[derive(Debug)]
    pub struct EquivalenceClass {
        class_name: String,
        class_state_ids: HashSet<String>,
    }

    impl EquivalenceClass {
        // Static counter to keep track of the next available class number

        pub fn new(class_state_ids: HashSet<String>) -> EquivalenceClass {
            // Increment the class counter
            let class_number = Self::get_next_class_number();
            let class_name = Self::get_name(class_number);
            EquivalenceClass {
                class_name,
                class_state_ids,
            }
        }
        pub fn is_empty(&self) -> bool {
            self.class_state_ids.is_empty()
        }

        pub fn get_name(class_number: u32) -> String {
            format!("{}{}", NAME, class_number)
        }

        pub fn state_ids(&self) -> &HashSet<String> {
            &self.class_state_ids
        }

        pub fn class_name(&self) -> &String {
            &self.class_name
        }

        pub fn get_current_num() -> u32 {
            unsafe {
                if CLASS_COUNTER == 0 {
                    CLASS_COUNTER
                } else {
                    CLASS_COUNTER - 1
                }
            }
        }

        // Helper method to get the next available class number
        fn get_next_class_number() -> u32 {
            unsafe {
                let class_number = CLASS_COUNTER;
                CLASS_COUNTER += 1;
                class_number
            }
        }
    }
}
