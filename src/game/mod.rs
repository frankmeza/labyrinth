use crate::{ascii, story};

fn print_start_message() {
    println!("{}", story::star_separator());
    println!("{}", story::lost_in_a_labyrinth());
    println!("{}", ascii::minotaur());
    println!("{}", story::star_separator());
}

pub fn run() {
    print_start_message();
}