use crate::game;
use std::process;

pub fn handle_choice(choice: &str, is_valid_choice: &mut bool) {
    match choice {
        "1" => {
            println!("entering game");
            *is_valid_choice = true;
            game::run();
        }
        "2" => {
            println!("quitting game");
            *is_valid_choice = true;
            process::exit(0);
        }
        _ => {
            println!("invalid");
        }
    }
}
