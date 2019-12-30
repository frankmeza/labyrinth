use crate::{ascii, map::Map, player::Player, story};
use std::process;

pub struct Game {}

impl Game {
    fn print_start_message() {
        println!("{}", story::star_separator());
        println!("{}", story::lost_in_a_labyrinth());
        println!("{}", ascii::minotaur());
        println!("{}", story::star_separator());
    }

    pub fn run(player: &mut Player) {
        Game::print_start_message();

        let map = Map::new();
        map.enter_labyrinth(player);
    }

    pub fn quit() {
        process::exit(0);
    }
}
