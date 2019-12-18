use std::io;

mod ascii;
mod game;
mod menu;
mod story;
mod utils;

fn main() {
    menu::display();
    let mut is_valid_choice = false;

    while !is_valid_choice {
        let mut menu_choice = String::new();
        io::stdin().read_line(&mut menu_choice).unwrap().to_string();

        let choice: &str = menu_choice.as_str().trim();
        utils::handle_choice(&choice, &mut is_valid_choice);
    }
}
