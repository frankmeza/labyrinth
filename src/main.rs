use std::io;

mod ascii;
mod constants;
mod game;
mod item;
mod map;
mod menu;
mod player;
mod space;
mod story;
mod utils;

use game::Game;
use player::Player;

fn main() {
    println!("{}", menu::show_options());

    let mut is_valid_choice = false;

    let player = Player::new();

    while !is_valid_choice {
        let mut menu_choice = String::new();
        io::stdin().read_line(&mut menu_choice).unwrap().to_string();

        let choice: &str = menu_choice.as_str().trim();
        let will_continue = utils::handle_choice(&choice, &mut is_valid_choice);

        if is_valid_choice {
            if will_continue {
                Game::run(&player);
            } else {
                Game::quit();
            }
        }
    }
}
