use crate::{constants, player::Player, story};
use std::collections::HashMap;

mod empty;
mod item;
mod minotaur;

pub use empty::EmptySpace;
pub use item::ItemSpace;
pub use minotaur::MinotaurSpace;

pub struct Space {
    pub description: String,
    pub exits: HashMap<usize, usize>,
}

pub enum SpaceType {
    Empty(EmptySpace),
    Item(ItemSpace),
    Minotaur(MinotaurSpace),
}

pub trait Room {
    // fn do_menu(player: &Player) -> bool;
    fn do_menu(&self, player: &Player) -> bool;
    fn has_items() -> bool;
}

impl Space {
    fn new(description: String) -> Self {
        let exits = self::exits(&description);
        Space { description, exits }
    }
}

impl SpaceType {
    pub fn get_space_exits(&self) -> &HashMap<usize, usize> {
        match self {
            SpaceType::Empty(e) => &e.space.exits,
            SpaceType::Item(i) => &i.space.exits,
            SpaceType::Minotaur(m) => &m.space.exits,
        }
    }

    pub fn get_room_name(&self) -> String {
        match self {
            SpaceType::Empty(e) => String::from(&e.space.description),
            SpaceType::Item(i) => String::from(&i.space.description),
            SpaceType::Minotaur(m) => String::from(&m.space.description),
        }
    }

    pub fn get_room_menu(&self, player: &Player) -> String {
        match self {
            SpaceType::Empty(space) => String::from(&space.do_menu(player)),
            SpaceType::Item(i) => String::from(&i.space.do_menu()),
            SpaceType::Minotaur(m) => String::from(&m.space.do_menu()),
        }
    }
}

pub fn exits(description: &str) -> HashMap<usize, usize> {
    match description {
        constants::STARTING_ROOM => {
            let mut e = HashMap::new();
            e.insert(0, 1);
            e.insert(1, 2);
            e.insert(2, 4);
            e.insert(3, 5);
            e
        }
        constants::ROOM_1 => {
            let mut e = HashMap::new();
            e.insert(1, 3);
            e.insert(2, 0);
            e
        }
        constants::ROOM_2 => {
            let mut e = HashMap::new();
            e.insert(0, 3);
            e.insert(2, 6);
            e.insert(3, 0);
            e
        }
        constants::ROOM_3 => {
            let mut e = HashMap::new();
            e.insert(2, 3);
            e.insert(3, 1);
            e
        }
        constants::ROOM_4 => {
            let mut e = HashMap::new();
            e.insert(0, 0);
            e.insert(1, 6);
            e.insert(3, 7);
            e
        }
        constants::ROOM_5 => {
            let mut e = HashMap::new();
            e.insert(1, 0);
            e.insert(2, 7);
            e
        }
        constants::ROOM_6 => {
            let mut e = HashMap::new();
            e.insert(0, 2);
            e.insert(3, 4);
            e
        }
        constants::FINAL_ROOM => {
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
