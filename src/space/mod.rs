use crate::player::Player;
use crate::story;
use std::collections::HashMap;

mod empty;
mod item;
mod minotaur;

pub use empty::EmptySpace;
pub use item::ItemSpace;
pub use minotaur::MinotaurSpace;

pub trait Space {
    fn new(description: String) -> Self;
    fn do_menu(player: &Player) -> bool;
    fn has_items() -> bool;
}

pub fn exits(description: &str) -> HashMap<usize, usize> {
    match description {
        "Starting Room" => {
            let mut e = HashMap::new();
            e.insert(0, 1);
            e.insert(1, 2);
            e.insert(2, 4);
            e.insert(3, 5);
            e
        }
        "Room 1" => {
            let mut e = HashMap::new();
            e.insert(1, 3);
            e.insert(2, 0);
            e
        }
        "Room 2" => {
            let mut e = HashMap::new();
            e.insert(0, 3);
            e.insert(2, 6);
            e.insert(3, 0);
            e
        }
        "Room 3" => {
            let mut e = HashMap::new();
            e.insert(2, 3);
            e.insert(3, 1);
            e
        }
        "Room 4" => {
            let mut e = HashMap::new();
            e.insert(0, 0);
            e.insert(1, 6);
            e.insert(3, 7);
            e
        }
        "Room 5" => {
            let mut e = HashMap::new();
            e.insert(1, 0);
            e.insert(2, 7);
            e
        }
        "Room 6" => {
            let mut e = HashMap::new();
            e.insert(0, 2);
            e.insert(3, 4);
            e
        }
        "Final Room" => {
            let mut e = HashMap::new();
            e.insert(0, 5);
            e.insert(1, 4);
            e
        }
        _ => {
            let mut e = HashMap::new();
            e.insert(0, 0);
            e
        }
    }
}

pub fn get_exit_options(space_exits: &HashMap<usize, usize>) -> String {
    let mut exit_options = String::from("");
    let mut exits: Vec<usize> = vec![];

    // todo put the keys into order before printing
    for (option, _room) in space_exits {
        exits.push(*option);
    }
    exits.sort();

    for e in 0..exits.len() {
        let option = story::get_exit_options(&e);
        exit_options.push_str(&option);
        exit_options.push_str("\n");
    }

    exit_options
}
