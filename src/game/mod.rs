use crate::{ascii, story, map::Map, player::Player};
use std::process;

pub struct Game {}

impl Game {
    fn print_start_message() {
        println!("{}", story::star_separator());
        println!("{}", story::lost_in_a_labyrinth());
        println!("{}", ascii::minotaur());
        println!("{}", story::star_separator());
    }

    pub fn run(player: &Player) {
        Game::print_start_message();

        let map = Map::new();
        map.enter_labyrinth(player);
    }

    pub fn quit() {
        process::exit(0);
    }
}
