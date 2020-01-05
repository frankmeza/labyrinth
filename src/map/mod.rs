use crate::{
    ascii, constants as c,
    player::Player,
    space::{self, EmptySpace, ItemSpace, MinotaurSpace, Space, SpaceType},
};

pub struct Map {
    pub spaces: [SpaceType; 8],
}

impl Map {
    pub fn new() -> Self {
        let spaces = Map::generate_map_spaces();
        Map { spaces }
    }

    pub fn get_space(&self, index: usize) -> &SpaceType {
        &self.spaces[index]
    }

    pub fn get_space_by_name(&self, room_name: String) -> &Space {
        let mut iter = self.spaces.iter();
        let found_space = &iter.find(|&st| st.get_room_name() == room_name);

        // TODO handle this better
        match found_space {
            None => self.get_space(0).get_space(),
            Some(space_type) => space_type.get_space(),
        }
    }

    pub fn enter_labyrinth(&self, player: &mut Player) {
        let mut starting_room = self.get_space(0);
        Map::handle_arrive_in_room(&self, &mut starting_room, player);
    }

    pub fn handle_arrive_in_room(&self, room: &mut SpaceType, player: &mut Player) {
        let space = self.get_space_by_name(player.get_current_room());

        println!("{}", &space.get_art());
        println!("{}\n\n", &space.get_description());
        println!("{}\n", ascii::lit_torch());
        println!("{}", space::get_exit_options(&space.exits));

        player.set_current_room(&room.get_room_name());
        space.do_menu(player);
    }

    fn generate_map_spaces() -> [SpaceType; 8] {
        let space_types: [SpaceType; 8] = [
            SpaceType::Empty(EmptySpace::new(String::from(c::STARTING_ROOM))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_1))),
            SpaceType::Empty(EmptySpace::new(String::from(c::ROOM_2))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_3))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_4))),
            SpaceType::Empty(EmptySpace::new(String::from(c::ROOM_5))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_6))),
            SpaceType::Minotaur(MinotaurSpace::new(String::from(c::FINAL_ROOM))),
        ];

        space_types
    }
}
