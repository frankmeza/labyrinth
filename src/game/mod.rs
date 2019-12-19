use crate::{ascii, story};
use std::process;

pub struct Game {}

impl Game {
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
