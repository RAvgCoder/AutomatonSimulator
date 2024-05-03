use std::fs::File;
use std::io;
use std::io::Read;

use crate::automaton_graph::AutomatonType;
use crate::automaton_graph::AutomatonType::{DFA, NFA, PDA};

mod automaton_graph;
mod dfa;
mod menus;
mod parser;

fn main() {
    println!("What is the absolute file path which contains the description of the automaton?");
    let automaton_program = read_program();

    println!("What type of automaton is this");
    // let automaton_type = read_automaton_type();
    let automaton_type = DFA;

    let menu_idx = menus::find_command_from_menu(match automaton_type {
        DFA => menus::dfa_menu::list(),
        NFA => menus::nfa_menu::list(),
        PDA => menus::pda_menu::list(),
    });

    match automaton_type {
        DFA => menus::dfa_menu::table(menu_idx, automaton_program),
        NFA => menus::nfa_menu::table(menu_idx, automaton_program),
        PDA => menus::pda_menu::table(menu_idx, automaton_program),
    }
}

fn read_automaton_type() -> AutomatonType {
    let mut automaton_type = String::new();
    io::stdin()
        .read_line(&mut automaton_type)
        .expect("Failed to read the automaton type");

    let automaton_type = match automaton_type
        .trim_end_matches(end_of_line())
        .to_uppercase()
        .as_str()
    {
        "DFA" => DFA,
        "NFA" => NFA,
        "PDA" => PDA,
        other => {
            panic!(
                "Cannot recognise automaton {} for types {:?}",
                other,
                vec![NFA, DFA, PDA]
            );
        }
    };
    automaton_type
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
    File::open(file_path.trim_end_matches(end_of_line()))
        .expect(&format!("Could not open the file {}", file_path))
        .read_to_string(&mut prog)
        .expect(&format!(
            "Could not read the program from the file {}",
            file_path
        ));

    prog
}

#[inline(always)]
fn end_of_line() -> &'static str {
    "\r\n"
}
