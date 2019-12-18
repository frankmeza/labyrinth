use std::io;
use std::process;

mod ascii;
mod game;
mod menu;
mod story;

fn handle_choice(choice: &str, is_valid_choice: &mut bool) {
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

fn main() {
    menu::display();
    let mut is_valid_choice = false;

    while !is_valid_choice {
        let mut menu_choice = String::new();
        io::stdin().read_line(&mut menu_choice).unwrap().to_string();

        let choice: &str = menu_choice.as_str().trim();
        handle_choice(&choice, &mut is_valid_choice);
    }
}
