use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

use regex::Regex;

use automaton_graph::State;

use crate::automaton_graph;
use crate::automaton_graph::{Automaton, AutomatonType, Position, Symbol, Tests, Transition};
use crate::automaton_graph::AutomatonType::{DFA, NFA, PDA};
use crate::parser::{Parser, ParserError, Scope, Separator, SkeletonState};
use crate::parser::parser::ParserError::{
    MissingObjSeparator, NoObjName, ObjNameMismatch, ObjNameNotFound, ObjNameOverFlow,
    ObjNameSyntaxErr,
};
use crate::parser::ParserError::{OutOfInput, ScopeError};


impl Parser {
    /// Parses the file that describes the automaton whose skeleton
    /// is described as or is provided by the website https://automatonsimulator.com/
    ///
    ///     {
    ///
    ///       "type": "DFA",
    ///
    ///       "dfa": {
    ///
    ///         "transitions": {},
    ///
    ///         "startState": "start",
    ///
    ///         "acceptStates": []
    ///
    ///       },
    ///
    ///       "states": {},
    ///
    ///       "transitions": [],
    ///
    ///       "bulkTests": {
    ///
    ///         "accept": "",
    ///
    ///         "reject": ""
    ///
    ///       }
    ///
    ///     }
    ///
    pub fn parse(program: String) -> Automaton {
        // Creates a parser for to parse the skeleton of the program
        let mut skeleton_parser = Self::new(Self::prog_preprocessor(program))
            .set_counter(1); // Read an opening scope at the beginning

        // Represents the order it expects the files
        // skeleton should be in when parsed
        let mut skeleton_states = [
            SkeletonState::Type,
            SkeletonState::AutomatonType,
            SkeletonState::States,
            SkeletonState::Transitions,
            SkeletonState::BulkTests,
        ]
            .iter();

        // Vars used to build the automaton
        let mut automaton_type: Option<AutomatonType> = None;
        let mut state_list: Vec<Rc<State>> = vec![];
        let mut accepting_strings: Vec<String> = vec![];
        let mut rejecting_strings: Vec<String> = vec![];

        // Start the parser
        while skeleton_parser.can_consume() {
            // Get the name of the skeleton section passed in
            let skeleton_state =
                Self::extract_skeleton_state_name(&mut skeleton_parser, &mut skeleton_states);

            // Read the separator
            skeleton_parser
                .try_consume_separator(Separator::COLUMN)
                .unwrap();

            // Check the state and act accordingly
            match skeleton_state {
                SkeletonState::Type => {
                    // Read the name of the automaton and match accordingly
                    let automaton = skeleton_parser
                        .try_consume_name()
                        .unwrap()
                        .to_ascii_uppercase();
                    automaton_type = Some(match automaton.as_str() {
                        "DFA" => DFA,
                        "NFA" => NFA,
                        "PDA" => PDA,
                        _ => panic!("Could not figure out the name of the automaton being used"),
                    });
                }
                SkeletonState::AutomatonType => {
                    let _ = skeleton_parser
                        .try_consume_scope(Scope::CurlyBracket)
                        .unwrap();
                }
                SkeletonState::States => {
                    let mut state_parser = skeleton_parser
                        .try_consume_scope(Scope::CurlyBracket)
                        .unwrap();

                    // Process each state
                    let mut iter_count = 0;
                    while state_parser.can_consume() {
                        iter_count += 1;
                        let state_name = state_parser.try_consume_name().unwrap();

                        let mut position = Position { x: 0.0, y: 0.0 };
                        let mut is_accept_state = false;
                        let mut alt_name: Option<String> = None;

                        state_parser
                            .try_consume_separator(Separator::COLUMN)
                            .unwrap();

                        // Parse the state info

                        let state_scope_parser = state_parser
                            .try_consume_scope(Scope::CurlyBracket)
                            .unwrap();

                        let mut display_id_cursor = state_scope_parser.cursor + 1;
                        state_scope_parser
                            .program_iter
                            .split(',')
                            .for_each(|line: &str| {
                                let info: Vec<&str> = line.split(':').collect::<Vec<&str>>();

                                /*  This is for a start states that is not an accepting state as
                                 *  it has an empty scope
                                 *  Eg: "start": {},
                                 */
                                if state_name != "start" {
                                    assert_eq!(info.len(), 2, "Info for the node is not a pair for iter count {}", iter_count);
                                    // Column + comma
                                    display_id_cursor += 2 + (info[0].len() + info[1].len()) as u32;
                                }


                                match info[0] {
                                    "\"isAccept\"" => is_accept_state = (*info[1]) == *"true",
                                    "\"top\"" => position.y = (*info[1]).parse::<f64>().unwrap(),
                                    "\"left\"" => position.x = (*info[1]).parse::<f64>().unwrap(),
                                    "\"displayId\"" => alt_name = Some(Self::new(info[1].to_string())
                                        // -1 used as there is no comma at the end
                                        .set_counter(display_id_cursor - 1 - info[1].len() as u32)
                                        .try_consume_name()
                                        .map_err(|parser_error: ParserError| {
                                            format!(
                                                "Cannot parse name for displayId resulted in an error {}",
                                                parser_error
                                            )
                                        })
                                        .unwrap()),
                                    _ => {
                                        if state_name != "start" || !info[0].is_empty() {
                                            panic!(
                                                "Unrecognized state property '{}' for state_name '{}'",
                                                info[0], state_name
                                            )
                                        }
                                    }
                                }
                            });

                        // Create the nodes and add them to a node list
                        state_list.push(Rc::new(State::new(
                            state_name.clone(),
                            alt_name.unwrap_or(state_name),
                            position,
                            is_accept_state,
                            RefCell::new(vec![]),
                        )));

                        let _ = state_parser.try_consume_separator(Separator::COMMA);
                    }
                }
                SkeletonState::Transitions => {
                    let mut transition_scope_parser = skeleton_parser
                        .try_consume_scope(Scope::BoxBracket)
                        .unwrap();

                    let mut state_a: Rc<State>;
                    let mut state_b: Rc<State>;
                    let mut label: Vec<char>;

                    let mut transition_iter_count = 1;
                    // Parse each transition
                    while transition_scope_parser.can_consume() {
                        // Get the info of the transition parsing
                        let mut transition_parser = transition_scope_parser
                            .try_consume_scope(Scope::CurlyBracket)
                            .unwrap();

                        let transition = transition_parser.try_consume_name().unwrap();
                        if "stateA" == transition // PDA name 
                            || "state_a" == transition
                        // DFA | NFA name
                        {
                            transition_parser
                                .try_consume_separator(Separator::COLUMN)
                                .unwrap();

                            let state_a_name = transition_parser.try_consume_name().unwrap();

                            // Find the state specified in the transition from the list of states
                            state_a = Self::find_state_by_id(&state_list, state_a_name.as_str())
                                .expect(&format!(
                                    "Cannot find state_a referenced in transition {}",
                                    state_a_name.as_str()
                                ))
                        } else {
                            panic!(
                                "Missing state_a transition name for transition. On iteration count {}",
                                transition_iter_count
                            )
                        }

                        let _ = transition_parser.try_consume_separator(Separator::COMMA);

                        if "label" == transition_parser.try_consume_name().unwrap().as_str() {
                            transition_parser
                                .try_consume_separator(Separator::COLUMN)
                                .unwrap();

                            // Parse the separator used
                            label = transition_parser
                                .try_consume_name()
                                .unwrap()
                                .split(',')
                                .into_iter()
                                .map(|str| {
                                    str.chars().nth(0).expect(&format!(
                                        "Cannot have an empty transition. On iteration count {}",
                                        transition_iter_count
                                    ))
                                })
                                .collect::<Vec<char>>();

                            // Check if the label is either a (DFA|NFA) Or a PDA
                            assert!(
                                label.len() == 1 || label.len() == 3,
                                "Cannot recognise transition label used. On iteration count {}",
                                transition_iter_count
                            );
                        } else {
                            panic!(
                                "Missing label transition name for transition. On iteration count {}",
                                transition_iter_count
                            )
                        }

                        let _ = transition_parser.try_consume_separator(Separator::COMMA);

                        let transition = transition_parser.try_consume_name().unwrap();
                        if "stateB" == transition  // PDA name  
                            || "state_b" == transition
                        // NFA | DFA name
                        {
                            transition_parser
                                .try_consume_separator(Separator::COLUMN)
                                .unwrap();

                            let state_b_name = transition_parser.try_consume_name().unwrap();

                            // Find the state specified in the transition from the list of states
                            state_b = Self::find_state_by_id(&state_list, state_b_name.as_str())
                                .expect(&format!(
                                    "Cannot find state_b referenced in transition {}. On iteration count {}",
                                    state_b_name.as_str(), transition_iter_count
                                ))
                        } else {
                            panic!(
                                "Missing state_b transition name for transition. On iteration count {}",
                                transition_iter_count
                            )
                        }

                        // Resolve label symbols
                        let mut push: Option<Symbol> = None;
                        let mut symbol: Symbol;
                        let mut pop: Option<Symbol> = None;

                        // Applicable for NFAs, DFAs & PDAs
                        symbol = if label[0] == 'ϵ' {
                            Symbol::EPSILON
                        } else {
                            Symbol::CHAR(label[0])
                        };

                        // For PDAs only
                        if label.len() == 3 {
                            symbol = if label[0] == 'ϵ' {
                                Symbol::EPSILON
                            } else {
                                Symbol::CHAR(label[0])
                            };
                            pop = Some(if label[1] == 'ϵ' {
                                Symbol::EPSILON
                            } else {
                                Symbol::CHAR(label[1])
                            });
                            push = Some(if label[2] == 'ϵ' {
                                Symbol::EPSILON
                            } else {
                                Symbol::CHAR(label[2])
                            });
                        }

                        // Create the transition
                        state_a.add_transition(Transition::new(state_b, symbol, pop, push));

                        let _ = transition_scope_parser.try_consume_separator(Separator::COMMA);
                        transition_iter_count += 1;
                    }
                }
                SkeletonState::BulkTests => {
                    let mut bulk_test_parser = skeleton_parser
                        .try_consume_scope(Scope::CurlyBracket)
                        .unwrap();

                    // Find all accepting strings
                    accepting_strings =
                        if "accept" == bulk_test_parser.try_consume_name().unwrap().as_str() {
                            bulk_test_parser
                                .try_consume_separator(Separator::COLUMN)
                                .unwrap();

                            // Parse accepting strings
                            bulk_test_parser
                                .try_consume_name()
                                .unwrap()
                                .split("\\n")
                                .map(|accepting_strings| String::from(accepting_strings))
                                .collect::<Vec<String>>()
                        } else {
                            panic!("No Accepting strings found")
                        };

                    let _ = bulk_test_parser.try_consume_separator(Separator::COMMA);

                    // Find all rejecting strings
                    rejecting_strings =
                        if "reject" == bulk_test_parser.try_consume_name().unwrap().as_str() {
                            bulk_test_parser
                                .try_consume_separator(Separator::COLUMN)
                                .unwrap();

                            // Parse rejecting strings
                            bulk_test_parser
                                .try_consume_name()
                                .unwrap()
                                .split("\\n")
                                .map(|rejecting_strings| String::from(rejecting_strings))
                                .collect::<Vec<String>>()
                        } else {
                            panic!("No rejecting strings found")
                        };
                }
                _ => panic!("User state can't be matched for {:?}", skeleton_state),
            }

            let _ = skeleton_parser.try_consume_separator(Separator::COMMA);
        }

        // Final checks
        {
            assert_eq!(
                skeleton_parser.program_iter.len(),
                0,
                "Parser was not empty after reading the program"
            );

            assert_eq!(
                skeleton_states.next(),
                None,
                "Not all fields were provided in the file"
            );
        }

        println!();
        dbg!(&accepting_strings);
        dbg!(&rejecting_strings);
        dbg!(&state_list);
        dbg!(skeleton_parser.cursor);
        dbg!(state_list.len());

        // Build the final automaton
        Automaton::new(
            // This should never fail as long as the skeleton_sate is correctly implemented
            // and the match on sate_type is also correct
            automaton_type.expect("Automaton type was never set"),
            Self::find_state_by_id(&state_list, "start").expect("No Start state found"),
            false,
            state_list // Create a list of all accepting states
                .iter()
                .filter(|node| node.is_accept_state)
                .cloned()
                .collect::<Vec<Rc<State>>>(),
            state_list,
            Tests {
                accepting_strings,
                rejecting_strings,
            },
        )
    }

    /// Creates a new Parser from a program string
    fn new(program: String) -> Parser {
        Parser {
            program_iter: program,
            cursor: 0,
        }
    }

    fn set_counter(mut self, val: u32) -> Self {
        self.cursor = val;
        self
    }

    /// Checks if the parser can still be used
    fn can_consume(&self) -> bool {
        self.program_iter.len() != 0
    }

    /// Define a function to search for a node by its ID
    fn find_state_by_id(states: &Vec<Rc<State>>, target_id: &str) -> Option<Rc<State>> {
        states.iter().find(|node| node.id == target_id).cloned()
    }

    /// Extracts object name from the parser
    fn extract_skeleton_state_name(
        parser: &mut Parser,
        obj_name_state: &mut Iter<SkeletonState>,
    ) -> SkeletonState {
        let skeleton_state_name = parser.try_consume_name().unwrap();

        // Find the current state to parse from the input
        let skeleton_state = SkeletonState::from_string(&skeleton_state_name).expect(&format!(
            "{}",
            ObjNameNotFound(format!(
                "Object name found is not a recognisable. Object name found is '{}'",
                skeleton_state_name
            ))
        ));

        // Find the expected state to parse
        let parsers_expected_state = obj_name_state.next()
            .expect(&format!("{}", ObjNameOverFlow(format!(
                "Tried to parse object name but has now exceeded number of states parsable state tried parsing was {}", skeleton_state_name
            ))));

        // Makes sure states match
        assert_eq!(
            skeleton_state,
            *parsers_expected_state,
            "{}",
            ObjNameMismatch(format!(
                "Object name trying to parse differs from the expected\n\
Expected : {:?}\n\
Found: {:?}",
                *parsers_expected_state, skeleton_state
            ))
        );

        skeleton_state
    }

    /// Try to consume the name of the objet to be parsed
    /// "anything in quotation marks"
    ///
    /// Eg: "type": "PDA",
    ///
    /// "type" in this case is the object name
    fn try_consume_name(&mut self) -> Result<String, ParserError> {
        let quotation_marks = '"';
        let mut prog_iter = self.program_iter.chars().peekable();

        // Tries to consume the starting quotation marks
        let first_char = prog_iter
            .next()
            .ok_or(OutOfInput(
                "Tried to read object name but no input is found".to_string(),
            ))
            .unwrap();

        // Validate that you can start reading the name of the object
        if first_char != quotation_marks {
            return Err(NoObjName(format!(
                "Expected to parse a quotation but found \"{}\" at index {}",
                first_char, self.cursor
            )));
        }

        // Len of the remaining prog to parse
        let remaining_prog_len = self.program_iter.len() - 1;

        // Create object name and consumes the closing quotes
        // reason why? refer to
        // https://www.reddit.com/r/rust/comments/x05yn5/make_take_while_not_consume_last_element_where_it/
        let string_retrieved = prog_iter
            .by_ref()
            .take_while(|&c| c != quotation_marks)
            .collect::<String>();

        // + 2 for the open and closing quotation
        let next_cursor_pos = self.cursor + string_retrieved.len() as u32 + 2;

        // No closing quotation was found
        if string_retrieved.len() == remaining_prog_len
        {
            Err(ObjNameSyntaxErr(format!(
                "Missing object name where expected. Error at prog index range {:?} But found {}",
                [(self.cursor)..(next_cursor_pos)],
                string_retrieved
            )))
        } else {
            // If parsing succeeded
            // Replace the string with the leftover strings in the iterator
            self.program_iter = prog_iter.collect::<String>();

            // Advance the cursor read
            self.cursor = next_cursor_pos;

            Ok(string_retrieved)
        }
    }

    /// Consumes a [Separator] from the input
    fn try_consume_separator(&mut self, separator: Separator) -> Result<(), ParserError> {
        let mut prog_iter = self.program_iter.chars();

        if let Some(c) = prog_iter.next() {
            if c == separator.into() {
                self.program_iter = prog_iter.collect::<String>();
                self.cursor += 1;
                return Ok(());
            }
        }

        Err(MissingObjSeparator(format!(
            "Missing separator {:?} found at prog_index {}",
            separator, self.cursor
        )))
    }

    /// Consumes a scope returning a parser that iterates over its contents
    /// Scope is anything withing the brackets described in the [Scope] struct
    fn try_consume_scope(&mut self, scope: Scope) -> Result<Parser, ParserError> {
        let mut prog_iter = self.program_iter.chars().peekable();

        // Tries to consume the starting quotation marks
        let opening_scope_char = prog_iter
            .next()
            .ok_or(ScopeError(
                "Tried to parse scope, but no input was found".to_string(),
            ))
            .unwrap();

        // Validate that you can start reading the name of the object
        if opening_scope_char != scope.into() {
            return Err(ScopeError(format!(
                "Expected to parse an opening scope {:?} but found '{}' at index {}",
                scope, opening_scope_char, self.cursor
            )));
        }

        // Len of the remaining prog to parse
        let remaining_prog_len = self.program_iter.len() - 1;

        let mut scope_counter = 1; // Initialized at 1 because the opening scope has been read
        // Collects scope contents
        let inner_scope_content = prog_iter
            .by_ref()
            .take_while(|&c| {
                if c == scope.into() {
                    scope_counter += 1;
                    true // Cannot close scope here
                } else if c == scope.closing() {
                    scope_counter -= 1;
                    // Possible that this can be a final closing bracket
                    scope_counter != 0
                } else {
                    true // Other characters
                }
            })
            .collect::<String>();

        // A snapshot of the previous cursor position before reading the scope
        let start_scope_cursor = self.cursor;

        // Retrieve the remaining program after the scope
        self.program_iter = prog_iter.collect::<String>();

        // + 2 for the open and closing scope
        self.cursor += inner_scope_content.len() as u32 + 2;

        // Missing end scope
        if inner_scope_content.len() == remaining_prog_len {
            return Err(ScopeError("No closing scope was found".parse().unwrap()));
        }

        Ok(
            Parser::new(inner_scope_content)
                .set_counter(start_scope_cursor + 1)
        )
    }

    /// Performs a full cleanup on the input program to remove new lines and spaces
    fn prog_preprocessor(program: String) -> String {
        // Remove new lines and spaces
        let cleaned_up_string = Regex::new(r"[\r\n\s*]")
            .expect("Failed to create program preprocessor regex")
            .replace_all(&program, "")
            .to_string();

        let mut prog = cleaned_up_string.chars();

        assert_eq!(
            prog.nth(0),
            Some('{'),
            "Program doesnt start with opening curly braces"
        );
        assert_eq!(
            prog.last(),
            Some('}'),
            "Program doesnt emd with closing curly braces"
        );

        // Remove the open and closing curly
        let new_prog = cleaned_up_string[1..(cleaned_up_string.len() - 1)].to_string();

        assert_ne!(
            new_prog.len(),
            0,
            "Invalid program as Program given is empty"
        );

        new_prog
    }
}
