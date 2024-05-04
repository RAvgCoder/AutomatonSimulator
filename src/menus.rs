use std::io;

pub mod pda_menu {
    pub fn list() -> Box<[&'static str]> {
        todo!()
    }

    pub fn table(command: u8, program: String) {}
}

pub mod nfa_menu {
    pub fn list() -> Box<[&'static str]> {
        todo!()
    }

    pub fn table(command: u8, program: String) {}
}

pub mod dfa_menu {
    use crate::dfa::DFA;

    pub fn list() -> Box<[&'static str]> {
        let x = ["Reduce Dfa"];
        Box::new(x)
    }

    pub fn table(command: u8, program: String) {
        let dfa = DFA::new(program);

        match command {
            1 => dfa.reduce(),
            _ => panic!("Error parsing command {}", command),
        }
    }
}

pub fn find_command_from_menu(menu: Box<[&str]>) -> u8 {
    println!("Which of these operations do you want to perform");
    for (idx, command) in menu.iter().enumerate() {
        println!("{}):\t{}", idx + 1, command)
    }

    let mut command_idx = String::new();
    io::stdin()
        .read_line(&mut command_idx)
        .expect("Could not read command to be performed");

    let command_idx = command_idx
        .trim_end_matches(end_of_line())
        .parse::<u8>()
        .map_err(|why| format!("Could not parse command to a number {}", why))
        .unwrap_or_else(|err| panic!("{}", err));

    if command_idx <= 0 || command_idx > menu.len() as u8 {
        panic!("Invalid command {}", command_idx);
    }

    command_idx
}

#[inline(always)]
fn end_of_line() -> &'static str {
    "\r\n"
}
