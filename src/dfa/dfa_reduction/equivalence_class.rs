use std::collections::HashSet;

/// The number the counter starts from
const START_COUNT: u8 = 0;
/// Counter to keep track of the next available class number
static mut CLASS_COUNTER: u32 = START_COUNT as u32;

#[derive(Debug)]
pub struct EquivalenceClass {
    class_name: String,
    class_state_ids: HashSet<String>,
}

impl EquivalenceClass {
    /// Prefix-Name given to each equivalence class
    const PREFIX_NAME: char = 'C';

    /// Creates a new Equivalence Class
    ///
    /// # Arguments
    ///
    /// * `class_state_ids`: Set of states belonging to the class
    pub fn new(class_state_ids: HashSet<String>) -> EquivalenceClass {
        // Increment the class counter
        assert_ne!(
            class_state_ids.len(),
            0,
            "Cannot create an equivalence class with no states"
        );

        let class_name = Self::create_name(Self::get_next_class_number());
        EquivalenceClass {
            class_name,
            class_state_ids,
        }
    }

    /// Returns the name of this equivalence class
    pub fn name(&self) -> String {
        self.class_name.clone()
    }

    /// Returns a name given a class number
    ///
    /// # Arguments
    ///
    /// * `class_number`: The number to give as the post-fix to the name
    ///
    /// # Examples
    ///
    /// ```
    /// let name = EquivalenceClass::get_name(1);
    /// println!("{}", name); // "C1"
    /// ```
    pub fn create_name(class_number: u32) -> String {
        format!("{}{}", Self::PREFIX_NAME, class_number)
    }

    /// Returns a reference to a set of states contained within the class
    pub fn state_ids(&self) -> &HashSet<String> {
        &self.class_state_ids
    }

    /// Returns the prefix name used by the Equivalence Struct
    pub fn prefix_name(&self) -> &String {
        &self.class_name
    }

    /// Returns the original number for the first equivalence class created
    pub fn get_start_count() -> u32 {
        START_COUNT as u32
    }

    // Helper method to get the next available class number
    fn get_next_class_number() -> u32 {
        unsafe {
            let class_number = CLASS_COUNTER;
            CLASS_COUNTER += 1;
            class_number
        }
    }

    /// Given a state name it finds the corresponding equivalence class it corresponds to in the list provided
    ///
    /// # Arguments
    ///
    /// * `state_name`: The state you want find the class name for
    /// * `equiv_states_list`: List of equivalent states you want to search though
    pub fn find_equiv_class_name(
        state_name: &str,
        equiv_states_list: &Vec<EquivalenceClass>,
    ) -> String {
        equiv_states_list
            .iter()
            .find(|&equiv_class| equiv_class.state_ids().contains(state_name))
            .expect(&format!(
                "Could not find {} in any of the equivalence classes",
                state_name
            ))
            .prefix_name()
            .clone()
    }
}
