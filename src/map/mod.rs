use crate::{
    ascii, constants as c,
    player::Player,
    space::{self, EmptySpace, ItemSpace, MinotaurSpace, SpaceType},
};

pub struct Map {
    pub spaces: Vec<SpaceType>,
}

impl Map {
    pub fn new() -> Self {
        let spaces = Map::generate_map_spaces();
        Map { spaces }
    }

    pub fn get_space(&self, index: usize) -> &SpaceType {
        &self.spaces[index]
    }

    pub fn enter_labyrinth(&self, player: &mut Player) {
        let starting_room = self.get_space(0);
        Map::handle_arrive_in_room(&self, &starting_room, player);
    }

    pub fn handle_arrive_in_room(&self, room: &SpaceType, player: &mut Player) {
        let space = room.get_space();

        println!("{}", &space.get_art());
        println!("{}\n\n", &space.get_description());
        println!("{}\n", ascii::lit_torch());
        println!("{}", space::get_exit_options(&space.exits));

        space.do_menu(player);
    }

    fn generate_map_spaces() -> Vec<SpaceType> {
        vec![
            SpaceType::Empty(EmptySpace::new(String::from(c::STARTING_ROOM))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_1))),
            SpaceType::Empty(EmptySpace::new(String::from(c::ROOM_2))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_3))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_4))),
            SpaceType::Empty(EmptySpace::new(String::from(c::ROOM_5))),
            SpaceType::Item(ItemSpace::new(String::from(c::ROOM_6))),
            SpaceType::Minotaur(MinotaurSpace::new(String::from(c::FINAL_ROOM))),
        ]
    }
}
