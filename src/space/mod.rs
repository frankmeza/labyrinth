use crate::{ascii, constants as c, game::Game, item::Item, map::Map, menu, player::Player, story};
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

    fn is_minotaur_space(&self) -> bool {
        &self.description == c::FINAL_ROOM
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

            let mut input_2 = String::from("");
            io::stdin().read_line(&mut input_2).unwrap().to_string();
            println!("after input 2: {}", input_2);

            got_input = Space::handle_menu_selection(&input_2, player);
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
        println!("INPUT IS {}", input);
        match input.trim() {
            c::CHOICE_0 => {
                let space = space_type.get_space();
                let mut all_items_picked_up = false;

                if space.has_items() {
                    let player_can_add_item = player.inventory.len() < c::MAX_NUMBER_ITEMS;

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
            c::CHOICE_5 => {
                player.handle_player_torch();
                false
            }
            c::CHOICE_I => {
                if player.has_items() {
                    for item in player.inventory.iter() {
                        println!("{}", item);
                    }
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

    pub fn handle_menu_selection(input: &str, player: &mut Player) -> bool {
        let map = Map::new();
        let mut is_moving_to_room = true;
        let mut has_selected = true;

        // .trim() is necessary! see #1 at bottom
        let space_type = match input.trim() {
            c::CHOICE_1 => map.get_space(0),
            c::CHOICE_2 => map.get_space(1),
            c::CHOICE_3 => map.get_space(2),
            c::CHOICE_4 => map.get_space(3),
            _ => {
                is_moving_to_room = false;
                map.get_space(0)
            }
        };

        if !is_moving_to_room {
            has_selected = Space::handle_options_within_room(input, space_type, player);
        }

        let room_name = space_type.get_room_name();
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
            e.insert(2, 3);
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
        let option = menu::get_exit_options(&e);
        exit_options.push_str(&option);
        exit_options.push_str("\n");
    }

    exit_options
}

pub fn get_art(room_name: &str) -> String {
    match room_name.trim() {
        // 0, 2, 4 print_left_forward_right_room
        c::STARTING_ROOM | c::ROOM_2 | c::ROOM_4 => {
            ascii::left_forward_right_room()
        }
        // 1, 5 print_forward_right_room
        c::ROOM_1 | c::ROOM_5 => ascii::forward_right_room(),
        // 3, 6 print_left_forward_room
        c::ROOM_3 | c::ROOM_6 => ascii::left_forward_room(),
        // 7 print_left_right_room
        c::FINAL_ROOM => ascii::left_right_room(),
        _ => String::from("very virus"),
    }
}

// #1: http://danielnill.com/rust_tip_compairing_strings
