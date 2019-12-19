use std::io;

mod ascii;
mod game;
mod map;
mod menu;
mod player;
mod space;
mod story;
mod utils;

use game::Game;
use map::Map;
// use player::Player;

fn main() {
    menu::display();
    let mut is_valid_choice = false;

    // let player = Player::new();
    // let game = Game::new();
    let map = Map::new();

    while !is_valid_choice {
        let mut menu_choice = String::new();
        io::stdin().read_line(&mut menu_choice).unwrap().to_string();

        let choice: &str = menu_choice.as_str().trim();
        let will_continue = utils::handle_choice(&choice, &mut is_valid_choice);

        if is_valid_choice {
            if will_continue {
                Game::run();
            } else {
                Game::quit();
            }
        }

        // story takes over?
    }
}
