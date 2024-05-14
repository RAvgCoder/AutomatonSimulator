use std::collections::HashMap;
use std::rc::Rc;

use crate::automaton_graph::{State, Symbol};
use crate::dfa::DFA;
use crate::dfa::dfa_reduction::equivalence_class::EquivalenceClass;

/// Holds steps to Displays a step-by-step solution for reducing a dfa
#[derive(Debug)]
pub struct DFAReductionStepsRenderer<'a> {
    table_steps: Vec<String>,
    steps: Vec<String>,
    state_transition_map: &'a HashMap<String, Vec<String>>,
    transitions_name_map: Vec<Symbol>,
    padding_size: usize,
    reduction_was_successful: bool,
    class_name_dividing: String,
}

impl<'a> DFAReductionStepsRenderer<'a> {
    pub fn new(
        state_transition_map: &'a HashMap<String, Vec<String>>,
        first_division: &Vec<EquivalenceClass>,
        all_states: &Vec<Rc<State>>,
    ) -> DFAReductionStepsRenderer<'a> {
        assert!(
            first_division.len() <= 2,
            "First division was not properly split into final and no final states"
        );

        let padding_size = 7;

        DFAReductionStepsRenderer {
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
    ) {
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

impl<'a> std::fmt::Display for DFAReductionStepsRenderer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
