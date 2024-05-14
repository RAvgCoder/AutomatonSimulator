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

    // Helper method to get the next available class number
    fn get_next_class_number() -> u32 {
        unsafe {
            let class_number = CLASS_COUNTER;
            CLASS_COUNTER += 1;
            class_number
        }
    }
}
