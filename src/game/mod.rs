use crate::{ascii, player, story};
use std::process;

pub struct Game {
    player: player::Player,
}

impl Game {
    pub fn new() -> Self {
        let player = player::Player { torch_lit: true };

        Game { player }
    }

    fn print_start_message() {
        println!("{}", story::star_separator());
        println!("{}", story::lost_in_a_labyrinth());
        println!("{}", ascii::minotaur());
        println!("{}", story::star_separator());
    }

    pub fn run() {
        Game::print_start_message();
    }

    pub fn quit() {
        process::exit(0);
    }
}
