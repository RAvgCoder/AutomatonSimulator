use std::rc::Rc;

use crate::automaton_graph::{SimulationError, State, Symbol, Transition};
use crate::pda::PDA;

#[derive(Debug)]
struct PdaInstance<'a> {
    curr_state: Rc<State>,
    stack: Vec<Symbol>,
    sim_str: &'a str,
}

impl<'a> Clone for PdaInstance<'a> {
    fn clone(&self) -> Self {
        PdaInstance {
            curr_state: self.curr_state.clone(),
            stack: self.stack.clone(),
            sim_str: self.sim_str,
        }
    }
}

impl PDA {
    pub fn simulate(&mut self, mut simulating_string: String) -> Result<bool, SimulationError> {
        let mut pda_instances = vec![PdaInstance {
            curr_state: self.automaton_graph.get_start_state(),
            stack: vec![],
            sim_str: &simulating_string[..],
        }];

        while !pda_instances.is_empty() {
            let instance = pda_instances.remove(0);
            // dbg!(&instance);

            let sim_str = instance.sim_str;

            let transition_epsilon = Self::find_epsilon_transitions(&instance);

            // Accept state with an empty string and at an accept state
            if sim_str.is_empty() && instance.curr_state.is_accept_state {
                return Ok(true);
            }

            /*
               If your in a state where u have an empty transition but no epsilon transitions to use on
               then pick another instance
            */
            if transition_epsilon.is_empty() && sim_str.is_empty() {
                continue;
            }

            let transitions_epsilon = Self::find_epsilon_transitions(&instance);

            //  Generate all instances for all epsilon transitions
            for e in transitions_epsilon {
                pda_instances.push(PdaInstance {
                    curr_state: e.to(),
                    stack: instance.stack.clone(),
                    sim_str: &sim_str[..],
                })
            }

            if sim_str.is_empty() {
                continue;
            }

            let transitions_sym =
                Self::find_transitions_on_symbol(&instance, sim_str.as_bytes()[0] as char);

            //  Generate all instances for all non-epsilon transitions
            for t in transitions_sym {
                let mut stack = instance.stack.clone();

                // Pop from the stack
                if let Some(to_pop_sym) = t.pop_symbol() {
                    // Find symbol to pop
                    if to_pop_sym != Symbol::EPSILON {
                        if let Some(popped_sym) = stack.pop() {
                            // If the symbol popped from the stack is diff from what you wanted then the simulation ends
                            if popped_sym != to_pop_sym {
                                return Ok(false);
                            }
                        } else {
                            continue;
                        }
                    }
                } else {
                    panic!("No pop symbol attached to current transition {:?}", t)
                }

                // Push to the stack
                if let Some(symbol_to_push) = t.push_symbol() {
                    if symbol_to_push != Symbol::EPSILON {
                        stack.push(symbol_to_push)
                    }
                } else {
                    panic!("No pop symbol attached to current transition {:?}", t)
                }

                // Add to the stack the new instance pushed
                let new_instance = PdaInstance {
                    curr_state: t.to(),
                    stack,
                    sim_str: &sim_str[1..],
                };
                dbg!(&new_instance);
                pda_instances.push(new_instance);
            }
        }

        Ok(false)
    }

    fn find_transitions_on_symbol(instance: &PdaInstance, c: char) -> Vec<Transition> {
        let transitions =
            Transition::find_transition_by_symbol(instance.curr_state.get_transitions(), c);
        transitions
    }

    fn find_epsilon_transitions(instance: &PdaInstance) -> Vec<Transition> {
        Transition::find_epsilon_transitions(instance.curr_state.get_transitions())
    }

    fn pda_assert(
        transitions: Vec<Transition>,
        curr_symbol: char,
        symbol_index: usize,
        curr_state: &Rc<State>,
    ) -> Result<(), SimulationError> {
        return if transitions.is_empty() {
            Err(SimulationError::NoTransitionForSymbol(format!(
                "No transition symbol found for character {} at index {} on state {:#?}",
                curr_symbol, symbol_index, curr_state
            )))
        } else {
            Ok(())
        };
    }
}
