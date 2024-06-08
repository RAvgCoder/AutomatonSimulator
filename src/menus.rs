use std::io;

const END_LINE: &str = "\r\n";

pub fn find_command_from_menu(menu_option_list: &[MenuOptions]) -> MenuOptions {
    println!("Which of these operations do you want to perform");
    menu_option_list
        .iter()
        .enumerate()
        .for_each(|(idx, command)| println!("{}):\t{:?}", idx + 1, command));

    let mut command_idx = String::new();
    io::stdin()
        .read_line(&mut command_idx)
        .expect("Could not read command to be performed");

    let option_idx = command_idx
        .trim_end_matches(END_LINE)
        .parse::<usize>()
        .map_err(|why| format!("Could not parse command to a number {}", why))
        .unwrap_or_else(|err| panic!("{}", err));

    *menu_option_list
        .get(option_idx - 1)
        .expect(&format!("Invalid option {}", option_idx))
}

fn read_simulating_string() -> String {
    println!("Input string to simulate for: ");
    let mut simulating_string = String::new();
    io::stdin()
        .read_line(&mut simulating_string)
        .expect("Failed to read string to simulate");
    simulating_string = simulating_string
        .trim_end_matches(END_LINE)
        .parse()
        .unwrap();

    simulating_string
}

#[derive(Debug, Copy, Clone)]
pub enum MenuOptions {
    // DFA
    SimulateDFA,
    ReduceDFA,

    // PDA
    SimulatePDA,
    GenerateCorrespondingGrammar,

    // NFA
    SimulateNFA,
    SimplifyNFA,
    NFAtoRegex,
    NFAtoDFA,
    RegexToNFA,
}

pub mod pda_menu {
    use std::rc::Rc;

    use crate::automaton_graph::Automaton;
    use crate::menus::{read_simulating_string, MenuOptions};
    use crate::pda::PDA;

    const MENU_OPTIONS: [MenuOptions; 2] = [
        MenuOptions::SimulatePDA,
        MenuOptions::GenerateCorrespondingGrammar,
    ];

    pub(crate) const fn list<'a>() -> &'a [MenuOptions] {
        &MENU_OPTIONS
    }

    pub fn table(menu_option: MenuOptions, automaton: Automaton) {
        let mut pda = PDA::new(Rc::new(automaton));

        match menu_option {
            MenuOptions::SimulatePDA => {
                let prog_to_sim = read_simulating_string();
                println!(
                    "Simulation ended in {} state",
                    if pda.simulate(prog_to_sim).unwrap() {
                        "an accept"
                    } else {
                        "a reject"
                    }
                )
            }
            _ => panic!("{:?} not available for PDAs", menu_option),
        }
    }
}

pub mod nfa_menu {
    use crate::automaton_graph::Automaton;
    use crate::menus::MenuOptions;

    const MENU_OPTIONS: [MenuOptions; 5] = [
        MenuOptions::SimulateNFA,
        MenuOptions::SimplifyNFA,
        MenuOptions::NFAtoRegex,
        MenuOptions::NFAtoDFA,
        MenuOptions::RegexToNFA,
    ];

    pub(crate) const fn list<'a>() -> &'a [MenuOptions] {
        &MENU_OPTIONS
    }

    pub fn table(menu_option: MenuOptions, automaton: Automaton) {
        match menu_option {
            _ => panic!("{:?} not available for NFAs", menu_option),
        }
    }
}

pub mod dfa_menu {
    use crate::automaton_graph::Automaton;
    use crate::dfa::DFA;
    use crate::menus::{read_simulating_string, MenuOptions};

    const MENU_OPTIONS: [MenuOptions; 2] = [MenuOptions::ReduceDFA, MenuOptions::SimulateDFA];

    pub fn table(menu_option: MenuOptions, automaton: Automaton) {
        let dfa = DFA::new(automaton);

        match menu_option {
            MenuOptions::ReduceDFA => {
                if let Some(steps) = dfa.reduce() {
                    println!("{}", steps);
                }
            }
            MenuOptions::SimulateDFA => {
                println!(
                    "Simulation ended in {} state",
                    if dfa.simulate(&read_simulating_string()).unwrap() {
                        "an accept"
                    } else {
                        "a reject"
                    }
                )
            }
            _ => panic!("{:?} not available for DFAs", menu_option),
        };
    }

    pub(crate) const fn list<'a>() -> &'a [MenuOptions] {
        &MENU_OPTIONS
    }
}
