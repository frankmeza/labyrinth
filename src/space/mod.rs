use crate::{ascii, constants as c, game::Game, map::Map, menu, player::Player, story};
use std::{collections::HashMap, io};

// SIBLING MODULES //

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
    pub fn new(description: String) -> Self {
        let exits = get_exits(&description);
        let art = get_art(&description);
        let items = get_items(&description);

        Space {
            description,
            exits,
            items,
            art,
        }
    }

    // GETTERS //

    pub fn get_art(&self) -> String {
        String::from(&self.art)
    }

    pub fn get_description(&self) -> String {
        String::from(&self.description)
    }

    pub fn get_exits(&self) -> &HashMap<usize, usize> {
        &self.exits
    }

    pub fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    pub fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    fn is_minotaur_space(&self) -> bool {
        &self.description == c::FINAL_ROOM
    }

    // SETTERS //

    // pub fn remove_items_from_space(space: &Space) {

    // }

    pub fn add_item_to_space(&mut self, item_name: &str) {
        self.items.push(String::from(item_name));
    }

    fn handle_options_within_room(input: &str, mut map: Map, player: &mut Player) -> bool {
        let map_ref = Map::new();

        let space = Space::get_space_by_name(player.get_current_room(), &map_ref);
        let iter = &mut map_ref.spaces.iter();

        let found_index = &iter
            .position(|s| s.get_description() == player.get_current_room())
            .unwrap();

        match input.trim() {
            c::CHOICE_0 => {
                if map.spaces[*found_index].has_items() {
                    let player_can_add_item = player.inventory.len() < c::MAX_NUMBER_ITEMS;

                    if player_can_add_item {
                        println!("{}", get_exit_options(space.get_exits()));
                        let items_in_room = space.get_items();

                        for item in items_in_room.iter() {
                            player.add_item(&item);
                        }

                        map.remove_items_from_space(space);
                        println!("{}", story::all_items_picked_up());
                    } else {
                        // the player has no room for the item,
                        // and so it must remain here in this space.
                        println!("{}", story::player_cannot_pick_up_item());
                    }
                } else {
                    // TODO
                }

                false
            }
            c::CHOICE_5 => {
                player.handle_player_torch();
                false
            }
            c::CHOICE_I => {
                if player.has_items() {
                    story::player_currently_carrying();
                    menu::print_player_items(&player.get_items());
                }

                false
            }
            c::CHOICE_D => {
                // TODO
                // player.drop_item(name: &str)
                false
            }
            c::CHOICE_Q => {
                Game::quit();
                false
            }
            _ => {
                Game::quit();
                false
            }
        }
    }

    // ASSOCIATED FUNCTIONS //

    pub fn do_menu(player: &mut Player, map_ref: &Map) {
        let space = Space::get_space_by_name(player.get_current_room(), &map_ref);

        let mut got_input = false;
        let mut did_print_torch = true;

        while !got_input {
            if player.get_torch_lit() {
                if !did_print_torch {
                    println!("{}", ascii::lit_torch());
                    did_print_torch = false;
                }

                if space.has_items() {
                    // TODO do a ghetto check in here that the
                    // items in space don't already exist in player
                    menu::print_space_items(&space.get_items());
                }

                if player.has_items() {
                    menu::player_has_items();
                    // story::player_items();
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

            got_input = Space::handle_menu_selection(&input, player, space);
        }
    }

    pub fn get_space_by_name(room_name: String, map: &Map) -> &Space {
        let mut iter = map.spaces.iter();
        let found_space = &iter.find(|&st| st.get_description() == room_name);
        // TODO handle this better
        match found_space {
            None => map.get_space(0),
            Some(space) => space,
        }
    }

    // helper fn, acts as a closure in handle_menu_selection()
    fn get_space_by_index(index: usize, map: &Map, exits_map: HashMap<usize, usize>) -> &Space {
        let found_index = exits_map.get(&index);
        println!("EXITS MAP: {:?}", &exits_map);
        println!("INDEXXXX: {:?}", &index);

        match found_index {
            None => {
                println!("get_space_by_index very virus");
                map.get_space(0)
            }
            Some(index) => map.get_space(*index),
        }
    }

    pub fn handle_menu_selection(input: &str, player: &mut Player, space: &Space) -> bool {
        let exits_map = get_exits(&space.get_description());
        let map = Map::new();
        let map_ref = Map::new();

        // .trim() is necessary for io::stdin().read_line(&mut input), see #1 at bottom
        let (space, staying_in_room) = match input.trim() {
            c::CHOICE_1 => (Space::get_space_by_index(0, &map_ref, exits_map), false),
            c::CHOICE_2 => (Space::get_space_by_index(1, &map_ref, exits_map), false),
            c::CHOICE_3 => (Space::get_space_by_index(2, &map_ref, exits_map), false),
            c::CHOICE_4 => (Space::get_space_by_index(3, &map_ref, exits_map), false),
            _ => (Space::get_space_by_index(0, &map_ref, exits_map), true),
        };

        if staying_in_room {
            return Space::handle_options_within_room(input, map, player);
        }

        map.handle_arrive_in_room(&space, player);
        true
    }
}

pub fn get_exits(room_name: &str) -> HashMap<usize, usize> {
    match room_name {
        c::STARTING_ROOM => {
            let mut e = HashMap::new();
            e.insert(0, 1);
            e.insert(1, 2);
            e.insert(2, 4);
            e.insert(3, 5);
            e
        }
        c::ROOM_1 => {
            let mut e = HashMap::new();
            e.insert(1, 3);
            e.insert(2, 0);
            e
        }
        c::ROOM_2 => {
            let mut e = HashMap::new();
            e.insert(0, 3);
            e.insert(2, 6);
            e.insert(3, 0);
            e
        }
        c::ROOM_3 => {
            let mut e = HashMap::new();
            e.insert(2, 2);
            e.insert(3, 1);
            e
        }
        c::ROOM_4 => {
            let mut e = HashMap::new();
            e.insert(0, 0);
            e.insert(1, 6);
            e.insert(3, 7);
            e
        }
        c::ROOM_5 => {
            let mut e = HashMap::new();
            e.insert(1, 0);
            e.insert(2, 7);
            e
        }
        c::ROOM_6 => {
            let mut e = HashMap::new();
            e.insert(0, 2);
            e.insert(3, 4);
            e
        }
        c::FINAL_ROOM => {
            let mut e = HashMap::new();
            e.insert(0, 5);
            e.insert(1, 4);
            e
        }
        _ => {
            let mut exits = HashMap::new();
            exits.insert(0, 0);
            exits
        }
    }
}

pub fn get_items(room_name: &str) -> Vec<String> {
    match room_name {
        c::ROOM_1 => vec![String::from(c::MATCHES)],
        c::ROOM_3 => vec![String::from(c::SHIELD)],
        c::ROOM_4 => vec![String::from(c::HEALTH_POTION)],
        c::ROOM_6 => vec![String::from(c::BOW)],
        _ => vec![],
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
            None => exit_options.push_str("Space::get_exit_options very virus"),
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
        // 0, 2, 4 print_left_forward_right_room
        c::STARTING_ROOM | c::ROOM_2 | c::ROOM_4 => ascii::left_forward_right_room(),
        // 1, 5 print_forward_right_room
        c::ROOM_1 | c::ROOM_5 => ascii::forward_right_room(),
        // 3, 6 print_left_forward_room
        c::ROOM_3 | c::ROOM_6 => ascii::left_forward_room(),
        // 7 print_left_right_room
        c::FINAL_ROOM => ascii::left_right_room(),
        _ => String::from("Space::get_art is very virus"),
    }
}

// #1: http://danielnill.com/rust_tip_compairing_strings
