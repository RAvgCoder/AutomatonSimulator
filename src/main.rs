use std::fs::File;
use std::io::Read;

use crate::automaton_graph::AutomatonType::{DFA, NFA, PDA};
use crate::parser::Parser;

mod automaton_graph;
mod dfa;
mod menus;
mod parser;

const END_LINE: &str = "\r\n";

fn main() {
    println!("What is the absolute file path which contains the description of the automaton?");
    let automaton = Parser::parse(read_program());
    let automaton_type = automaton.automaton_type;

    let menu_idx = menus::find_command_from_menu(match automaton_type {
        DFA => menus::dfa_menu::list(),
        NFA => menus::nfa_menu::list(),
        PDA => menus::pda_menu::list(),
    });

    match automaton_type {
        DFA => menus::dfa_menu::table(menu_idx, automaton),
        NFA => menus::nfa_menu::table(menu_idx, automaton),
        PDA => menus::pda_menu::table(menu_idx, automaton),
    }
}

fn read_program() -> String {
    let mut file_path = String::new();
    file_path = String::from(r"C:\Users\egbor\Videos\Rust\AutomatonSimulator\src\TESTS\input.txt");
    // Read the file_path for the automaton input
    // io::stdin()
    //     .read_line(&mut file_path)
    //     .expect("Failed to read absolute file path");

    let mut prog = String::new();

    // Trim for windows input
    File::open(file_path.trim_end_matches(END_LINE))
        .expect(&format!("Could not open the file {}", file_path))
        .read_to_string(&mut prog)
        .expect(&format!(
            "Could not read the program from the file {}",
            file_path
        ));

    prog
}
