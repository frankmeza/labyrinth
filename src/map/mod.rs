use crate::{
    ascii, constants as c,
    player::Player,
    space::{self, Space},
};

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

        // TODO handle this better
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

    pub fn enter_labyrinth(&mut self, player: &mut Player) {
        let map_ref = Self::new();
        let starting_room = map_ref.get_space(0);
        Map::handle_arrive_in_room(self, starting_room, player);
    }

    pub fn handle_arrive_in_room(&mut self, room: &Space, player: &mut Player) {
        let space = self.get_space_by_name(room.get_description());

        println!("{}", &space.get_art());
        println!("{}\n\n", &space.get_description());
        println!("{}\n", ascii::lit_torch());
        println!("{}", space::get_exit_options(&space.exits));

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
