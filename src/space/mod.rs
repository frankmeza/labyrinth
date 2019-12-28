use crate::{ascii, constants, game::Game, item::Item, map::Map, menu, player::Player, story};
use std::{collections::HashMap, io};

mod empty;
mod item;
mod minotaur;

pub use empty::EmptySpace;
pub use item::ItemSpace;
pub use minotaur::MinotaurSpace;

#[derive(Debug)]
pub struct Space {
    pub description: String,
    pub exits: HashMap<usize, usize>,
    pub items: Vec<String>,
    pub art: String,
}

#[derive(Debug)]
pub enum SpaceType {
    Empty(EmptySpace),
    Item(ItemSpace),
    Minotaur(MinotaurSpace),
}

impl Space {
    fn new(description: String) -> Self {
        let exits = exits(&description);
        let art = get_art(&description);
        let items = vec![];

        Space {
            description,
            exits,
            items,
            art,
        }
    }

    pub fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    pub fn get_art(&self) -> String {
        String::from(&self.art)
    }

    pub fn get_description(&self) -> String {
        String::from(&self.description)
    }

    // pub fn get_exits(&self) -> &HashMap<usize, usize> {
    //     &self.exits
    // }

    fn is_minotaur_space(&self) -> bool {
        &self.description == constants::FINAL_ROOM
    }

    // self will check for is_minotaur
    pub fn do_menu(&self, player: &mut Player) {
        let mut got_input = false;
        let mut did_print_torch = true;

        while !got_input {
            if player.get_torch_lit() {
                if !did_print_torch {
                    println!("{}", ascii::lit_torch());
                    did_print_torch = false;
                }

                if self.has_items() {
                    Space::handle_space_has_items(&self.items);
                }

                if player.has_items() {
                    Player::handle_player_has_items();
                }

                println!("{}", menu::quit_game());
            } else {
                if player.has_item("matches") {
                    println!("{}", menu::can_relight_torch());
                } else {
                    println!("{}", story::cannot_relight_torch());
                    Game::quit();
                }
            }

            let mut input = String::from("");
            io::stdin().read_line(&mut input).unwrap().to_string();
            println!("after input: {}", input);

            got_input = Space::handle_menu_selection(&self, &input, player);
        }
    }

    //////////////////////////
    // ASSOCIATED FUNCTIONS //
    //////////////////////////

    fn handle_space_has_items(items: &Vec<String>) {
        println!("{}", story::items_on_ground());
        let items_map = Item::all_items();

        for name in items.iter() {
            let found_item = items_map.get(name);

            match found_item {
                None => println!("computer is very virus"),
                Some(item) => {
                    println!("{}", item.get_description());
                    println!("{}", item.get_art());
                }
            }
        }

        println!("{}", menu::pick_up_items());
    }

    fn handle_options_within_room(
        input: &str,
        space_type: &SpaceType,
        player: &mut Player,
    ) -> bool {
        match input.trim() {
            constants::CHOICE_0 => {
                let space = space_type.get_space();
                let mut all_items_picked_up = false;

                if space.has_items() {
                    let player_can_add_item = player.inventory.len() < constants::MAX_NUMBER_ITEMS;

                    if player_can_add_item {
                        // TODO
                        // space.remove_item(item_name)
                        // player.pick_up_item(item_name)
                        all_items_picked_up = true;
                    } else {
                        // the player has no room for the item,
                        // and it must remain here in this space
                        println!("{}", story::player_cannot_pick_up_item());
                    }
                } else {
                }

                if all_items_picked_up {
                    println!("{}", story::all_items_picked_up());
                }

                false
            }
            constants::CHOICE_5 => {
                player.handle_player_torch();
                false
            }
            constants::CHOICE_I => {
                if player.has_items() {
                    for item in player.inventory.iter() {
                        println!("{}", item);
                    }
                }

                false
            }
            constants::CHOICE_D => {
                // TODO
                // player.drop_item(name: &str)
                false
            }
            constants::CHOICE_Q => {
                Game::quit();
                false
            }
            _ => {
                Game::quit();
                false
            }
        }
    }

    // helper fn, acts as a closure in handle_menu_selection()
    fn get_space_by_index(index: usize, map: &Map, exits_map: HashMap<usize, usize>) -> &SpaceType {
        let found_index = exits_map.get(&index);
        match found_index {
            None => {
                println!("COMPUTER IS VERY VIRUS");
                map.get_space(0)
            }
            Some(index) => map.get_space(*index),
        }
    }

    pub fn handle_menu_selection(&self, input: &str, player: &mut Player) -> bool {
        let map = Map::new();
        let mut is_moving_to_room = true;
        let mut has_selected = true;

        let exits_map = exits(&self.get_description());

        // .trim() is necessary! see #1 at bottom
        let space_type = match input.trim() {
            constants::CHOICE_1 => Space::get_space_by_index(0, &map, exits_map),
            constants::CHOICE_2 => Space::get_space_by_index(1, &map, exits_map),
            constants::CHOICE_3 => Space::get_space_by_index(2, &map, exits_map),
            constants::CHOICE_4 => Space::get_space_by_index(3, &map, exits_map),
            _ => {
                is_moving_to_room = false;
                Space::get_space_by_index(0, &map, exits_map)
            }
        };

        if !is_moving_to_room {
            has_selected = Space::handle_options_within_room(input, space_type, player);
        }

        let room_name = self.get_description();

        player.set_current_room(&room_name);
        map.handle_arrive_in_room(space_type, player);

        has_selected
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

    pub fn get_room_name(&self) -> String {
        match self {
            SpaceType::Empty(e) => String::from(&e.space.description),
            SpaceType::Item(i) => String::from(&i.space.description),
            SpaceType::Minotaur(m) => String::from(&m.space.description),
        }
    }
}

pub fn exits(room_name: &str) -> HashMap<usize, usize> {
    match room_name {
        constants::STARTING_ROOM => {
            let mut exits = HashMap::new();
            exits.insert(0, 1);
            exits.insert(1, 2);
            exits.insert(2, 4);
            exits.insert(3, 5);
            exits
        }
        constants::ROOM_1 => {
            let mut exits = HashMap::new();
            exits.insert(1, 3);
            exits.insert(2, 0);
            exits
        }
        constants::ROOM_2 => {
            let mut exits = HashMap::new();
            exits.insert(0, 3);
            exits.insert(2, 6);
            exits.insert(3, 0);
            exits
        }
        constants::ROOM_3 => {
            let mut exits = HashMap::new();
            exits.insert(2, 3);
            exits.insert(3, 1);
            exits
        }
        constants::ROOM_4 => {
            let mut exits = HashMap::new();
            exits.insert(0, 0);
            exits.insert(1, 6);
            exits.insert(3, 7);
            exits
        }
        constants::ROOM_5 => {
            let mut exits = HashMap::new();
            exits.insert(1, 0);
            exits.insert(2, 7);
            exits
        }
        constants::ROOM_6 => {
            let mut exits = HashMap::new();
            exits.insert(0, 2);
            exits.insert(3, 4);
            exits
        }
        constants::FINAL_ROOM => {
            let mut exits = HashMap::new();
            exits.insert(0, 5);
            exits.insert(1, 4);
            exits
        }
        _ => {
            let mut exits = HashMap::new();
            exits.insert(0, 0);
            exits
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
        let found_exit = exits.get(e);

        match found_exit {
            None => exit_options.push_str("MENU IS VERY VIRUS"),
            Some(exit) => {
                let option = menu::get_exit_options(&exit);

                exit_options.push_str(&option);
                exit_options.push_str("\n");
            }
        }
    }

    exit_options
}

pub fn get_art(room_name: &str) -> String {
    match room_name.trim() {
        // index 0, 2, 4
        constants::STARTING_ROOM | constants::ROOM_2 | constants::ROOM_4 => {
            ascii::left_forward_right_room()
        }
        // index 1, 5
        constants::ROOM_1 | constants::ROOM_5 => ascii::forward_right_room(),
        // index 3, 6
        constants::ROOM_3 | constants::ROOM_6 => ascii::left_forward_room(),
        // index 7
        constants::FINAL_ROOM => ascii::left_right_room(),
        _ => String::from("very virus"),
    }
}

// #1: http://danielnill.com/rust_tip_compairing_strings
