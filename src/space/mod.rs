use crate::{ascii, constants, game::Game, item::Item, menu, player::Player, story};
use std::{collections::HashMap, io};

mod empty;
mod item;
mod minotaur;

pub use empty::EmptySpace;
pub use item::ItemSpace;
pub use minotaur::MinotaurSpace;

pub struct Space {
    pub description: String,
    pub exits: HashMap<usize, usize>,
    pub items: Vec<Item>,
}

pub enum SpaceType {
    Empty(EmptySpace),
    Item(ItemSpace),
    Minotaur(MinotaurSpace),
}

// on the chopping block?
pub trait Room {
    fn do_menu(&self, player: &Player) -> bool;
}

impl Space {
    fn new(description: String) -> Self {
        let exits = self::exits(&description);
        let items = vec![];

        Space {
            description,
            exits,
            items,
        }
    }

    pub fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    fn is_minotaur_space(&self) -> bool {
        &self.description == constants::FINAL_ROOM
    }

    fn handle_space_has_items(items: &Vec<Item>) {
        println!("{}", story::items_on_ground());

        for item in items.iter() {
            println!("{}", item.get_description());
            println!("{}", item.get_art());
        }

        println!("{}", menu::pick_up_items());
    }

    // empty and item will be exact duplicates
    // self will check for is minotaur
    pub fn do_menu(&self, player: &Player) -> bool {
        let mut got_input = false;

        let mut input = String::from("");
        io::stdin().read_line(&mut input).unwrap().to_string();

        while !got_input {
            if player.get_torch_lit() {
                println!("{}", ascii::lit_torch());

                if self.has_items() {
                    Space::handle_space_has_items(&self.items);
                }

                if player.has_items() {
                    Player::handle_player_has_items();
                }

                println!("{}", Game::quit_game());

                got_input = true // this is to be moved
            } else {
                if player.has_item("matches") {
                    println!("{}", menu::can_relight_torch());
                } else {
                    println!("{}", menu::cannot_relight_torch());
                    Game::quit();
                }
            }
        }

        true
    }
}

impl SpaceType {
    pub fn get_space(&self) -> &Space {
        match self {
            SpaceType::Empty(e) => &e.space,
            SpaceType::Item(i) => &i.space,
            SpaceType::Minotaur(m) => &m.space,
        }
    }

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
