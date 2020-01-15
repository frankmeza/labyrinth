#![allow(unused_mut)]
use crate::{ascii, constants as c, game::Game, menu, player::Player, space::Space, story};
use rand::distributions::{Distribution, Uniform};
use std::io;

#[derive(Debug)]
pub struct Map {
    pub spaces: [Space; 8],
}

impl Map {
    pub fn new() -> Self {
        let spaces = Map::generate_map_spaces();
        Map { spaces }
    }

    pub fn get_space(&self, index: usize) -> &Space {
        &self.spaces[index]
    }

    pub fn get_space_by_name(&self, room_name: String) -> &Space {
        let mut iter = self.spaces.iter();
        let found_space = &iter.find(|&st| st.get_description() == room_name);

        match found_space {
            None => self.get_space(0),
            Some(space) => space,
        }
    }

    pub fn remove_items_from_space(&mut self, space: &Space) {
        let mut iter = self.spaces.iter();
        let found_index = iter.position(|s| &s.get_description() == &space.get_description());

        match found_index {
            None => println!("remove_items_from_space is very virus"),
            Some(index) => self.spaces[index].items.clear(),
        }
    }

    pub fn add_item_to_space(&mut self, space: &Space, item_name: &str) {
        let mut iter = self.spaces.iter();
        let found_index = iter.position(|s| &s.get_description() == &space.get_description());

        match found_index {
            None => println!("remove_items_from_space is very virus"),
            Some(index) => {
                let space = &mut self.spaces[index];
                space.items.push(String::from(item_name));
            }
        }
    }

    pub fn enter_labyrinth(&mut self, player: &mut Player) {
        let map_ref = Self::new();
        let starting_room = map_ref.get_space(0);
        Map::handle_arrive_in_room(self, starting_room, player);
    }

    pub fn print_fight_prologue() {
        println!("{}", story::ready_to_fight());
        println!("{}", ascii::minotaur());
    }

    pub fn handle_arrive_in_room(&mut self, room: &Space, player: &mut Player) {
        let space = self.get_space_by_name(room.get_description());

        if space.get_description() == c::FINAL_ROOM {
            let player_items = player.get_items();

            if player_items.contains(&String::from(c::SHIELD))
                && player_items.contains(&String::from(c::BOW))
                && player_items.contains(&String::from(c::ARROWS))
            {
                println!("{}", menu::fight_the_minotaur());

                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap().to_string();

                match choice.trim() {
                    c::CHOICE_F => {
                        Map::print_fight_prologue();

                        if player_items.contains(&String::from(c::HEALTH_POTION)) {
                            println!("{}", story::you_have_health_potion());
                        }

                        story::enter_minotaur_lair();

                        let mut player_health = 50;
                        let mut minotaur_health = 50;

                        let mut rng = rand::thread_rng();
                        let die = Uniform::from(1..7);

                        // #![allow(unused_must_use)]
                        for _i in 0..4 {
                            let player_damage = die.sample(&mut rng);
                            let minotaur_damage = die.sample(&mut rng);

                            player_health - player_damage;
                            minotaur_health - minotaur_damage;
                        }

                        if player_health > minotaur_health {
                            story::you_killed_minotaur();
                            Game::quit();
                        }

                        if minotaur_health > player_health {
                            story::minotaur_killed_you();
                            Game::quit();
                        }
                    }
                    c::CHOICE_Q => {
                        Game::quit();
                    }
                    _ => (),
                }
            }
        }

        println!("{}", &space.get_art());
        println!("{}\n\n", &space.get_description());
        println!("{}\n", ascii::lit_torch());

        player.set_current_room(&room.get_description());
        Space::do_menu(player, self);
    }

    fn generate_map_spaces() -> [Space; 8] {
        let space_types: [Space; 8] = [
            Space::new(String::from(c::STARTING_ROOM)),
            Space::new(String::from(c::ROOM_1)),
            Space::new(String::from(c::ROOM_2)),
            Space::new(String::from(c::ROOM_3)),
            Space::new(String::from(c::ROOM_4)),
            Space::new(String::from(c::ROOM_5)),
            Space::new(String::from(c::ROOM_6)),
            Space::new(String::from(c::FINAL_ROOM)),
        ];

        space_types
    }
}
