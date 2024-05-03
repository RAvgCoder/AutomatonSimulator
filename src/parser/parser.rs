use std::slice::Iter;

use regex::Regex;

use crate::automaton_graph::AutomatonType::{DFA, NFA, PDA};
use crate::automaton_graph::{Automaton, AutomatonType};
use crate::parser::parser::ParserError::{
    MissingObjSeparator, NoObjName, ObjNameMismatch, ObjNameNotFound, ObjNameOverFlow,
    ObjNameSyntaxErr,
};
use crate::parser::utils::{Scope, Separator, State};
use crate::parser::ParserError::{OutOfInput, ScopeError};
use crate::parser::{Parser, ParserError};

impl Parser {
    /// Parses the file that describes the automaton whose skeleton
    /// is described as
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
    pub fn parse<'a>(program: String) -> Option<Automaton<'a>> {
        let mut parser = Self::new(prog_preprocessor(program));
        println!("\nProg to parse: {{\n{}\n}}\n", parser.program_iter);
        let mut obj_name_state = [
            State::Type,
            State::AutomatonType,
            State::States,
            State::Transitions,
            State::BulkTests,
        ]
        .iter();

        let mut automaton_type: AutomatonType;

        while parser.can_consume() {
            let user_state = Self::extract_obj_state(&mut parser, &mut obj_name_state);

            println!("State: {:?}", user_state);

            parser.consume_separator(Separator::COLUMN).unwrap();

            match user_state {
                State::Type => {
                    let automaton = parser.try_consume_name().unwrap().to_ascii_uppercase();

                    automaton_type = match automaton.as_str() {
                        "DFA" => DFA,
                        "NFA" => NFA,
                        "PDA" => PDA,
                        _ => panic!("Could not figure out the name of the automaton being used"),
                    };

                    println!("AutomatonType: {:?}", automaton_type);
                }
                State::AutomatonType => {
                    let _ = parser.consume_scope(Scope::Curly_Bracket);
                }
                State::States => {
                    let _ = parser.consume_scope(Scope::Curly_Bracket);
                }
                State::Transitions => {
                    let _ = parser.consume_scope(Scope::Box_Bracket);
                }
                State::BulkTests => {
                    let _ = parser.consume_scope(Scope::Curly_Bracket);
                }
                _ => panic!("User state can't be matched for {:?}", user_state),
            }

            let _ = parser.consume_separator(Separator::COMMA);
            println!("Program_cursor_position at index(Not zero indexed): {}", parser.cursor);
            println!()
        }

        assert_eq!(
            parser.program_iter.len(),
            0,
            "Parser was not empty after reading the program"
        );

        None
    }

    /// Extracts object name from the parser
    fn extract_obj_state(parser: &mut Parser, obj_name_state: &mut Iter<State>) -> State {
        let obj_name = parser.try_consume_name().unwrap();

        // Find the current state to parse from the input
        let user_state = State::from_string(&obj_name).expect(&format!(
            "{}",
            ObjNameNotFound(format!(
                "Object name found is not a recognisable. Object name found is '{}'",
                obj_name
            ))
        ));

        // Find the expected state to parse
        let parsers_expected_state = obj_name_state.next()
            .expect(&format!("{}", ObjNameOverFlow(format!(
                "Tried to parse object name but has now exceeded number of states parsable state tried parsing was {}", obj_name
            ))));

        // Makes sure states match
        assert_eq!(
            user_state,
            *parsers_expected_state,
            "{}",
            ObjNameMismatch(format!(
                "Object name trying to parse differs from the expected\n\
Expected : {:?}\n\
Found: {:?}",
                *parsers_expected_state, user_state
            ))
        );
        user_state
    }

    fn new(program: String) -> Parser {
        Parser {
            program_iter: program,
            cursor: 0,
        }
    }
    fn can_consume(&mut self) -> bool {
        self.program_iter.len() != 0
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
        let first_char = prog_iter.next().ok_or(OutOfInput(
            "Tried to read object name but no input is found".to_string(),
        ))?;

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
        let obj_name = prog_iter
            .by_ref()
            .take_while(|&c| c != quotation_marks)
            .collect::<String>();

        // + 2 for the open and closing quotation
        let next_cursor_pos = obj_name.len() as u32 + 2;

        if obj_name.len() == 0 // No name within the quotation
            // No closing quotation was found
            || obj_name.len() == remaining_prog_len
        {
            Err(ObjNameSyntaxErr(format!(
                "Missing object name where expected. Error at prog index range {:?}\nBut found {}",
                [(self.cursor + 1)..(next_cursor_pos)],
                obj_name
            )))
        } else {
            // If parsing succeeded
            // Replace the string with the leftover strings in the iterator
            self.program_iter = prog_iter.collect::<String>();
            // Advance the cursor read
            self.cursor += next_cursor_pos;

            Ok(obj_name)
        }
    }

    /// Consumes a column separator from the input
    fn consume_separator(&mut self, separator: Separator) -> Result<(), ParserError> {
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
    fn consume_scope(&mut self, scope: Scope) -> Result<Parser, ParserError> {
        let mut prog_iter = self.program_iter.chars().peekable();

        // Tries to consume the starting quotation marks
        let opening_scope_char = prog_iter.next().ok_or(ScopeError(
            "Tried to parse scope, but no input was found".to_string(),
        ))?;

        // Validate that you can start reading the name of the object
        if opening_scope_char != scope.into() {
            return Err(NoObjName(format!(
                "Expected to parse an opening scope {:?} but found \"{}\" at index {}",
                scope, opening_scope_char, self.cursor
            )));
        }

        // Len of the remaining prog to parse
        let remaining_prog_len = self.program_iter.len() - 1;

        let mut scope_checker = 1; // Initialized at 1 because the opening scope has been read
                                   // Collects scope contents
        let inner_scope_content = prog_iter
            .by_ref()
            .take_while(|&c| {
                if c == scope.into() {
                    scope_checker += 1;
                    true // Cannot close scope here
                } else if c == scope.closing() {
                    scope_checker -= 1;
                    // Possible that this can be a final closing bracket
                    scope_checker != 0
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

        if inner_scope_content.len() == (remaining_prog_len - 1)
        // Missing end scope
        {
            return Err(ScopeError("No closing scope was found".parse().unwrap()));
        }

        Ok(Parser {
            program_iter: inner_scope_content,
            cursor: start_scope_cursor,
        })
    }
}

fn prog_preprocessor(program: String) -> String {
    // Remove new lines and spaces
    let binding = Regex::new(r"[\r\n\s*]")
        .expect("Failed to create program preprocessor regex")
        .replace_all(&program, "")
        .to_string();

    let mut prog = binding.chars();

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
    binding[1..(binding.len() - 1)].to_string()
}
