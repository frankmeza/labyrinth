use crate::{
    ascii, constants,
    player::Player,
    space::{self, EmptySpace, ItemSpace, MinotaurSpace, SpaceType},
};

pub struct Map {
    pub spaces: Vec<SpaceType>,
}

fn generate_spaces() -> Vec<SpaceType> {
    vec![
        SpaceType::Empty(EmptySpace::new(String::from(constants::STARTING_ROOM))),
        SpaceType::Item(ItemSpace::new(String::from(constants::ROOM_1))),
        SpaceType::Empty(EmptySpace::new(String::from(constants::ROOM_2))),
        SpaceType::Item(ItemSpace::new(String::from(constants::ROOM_3))),
        SpaceType::Item(ItemSpace::new(String::from(constants::ROOM_4))),
        SpaceType::Empty(EmptySpace::new(String::from(constants::ROOM_5))),
        SpaceType::Item(ItemSpace::new(String::from(constants::ROOM_6))),
        SpaceType::Minotaur(MinotaurSpace::new(String::from(constants::FINAL_ROOM))),
    ]
}

impl Map {
    pub fn new() -> Self {
        let spaces = generate_spaces();
        Map { spaces }
    }

    fn get_space(&self, index: usize) -> &SpaceType {
        &self.spaces[index]
    }

    pub fn enter_labyrinth(&self, player: &Player) {
        let starting_room = self.get_space(0);
        let exits = starting_room.get_space_exits();

        println!("{}", ascii::left_forward_right_room());
        println!("{}\n\n", &starting_room.get_room_name());

        println!("{}\n", ascii::lit_torch());
        println!("{}", space::get_exit_options(exits));

        Map::handle_arrive_in_room(&self, &starting_room, player);
    }

    pub fn handle_arrive_in_room(&self, room: &SpaceType, player: &Player) {
        let space = room.get_space();
        space.do_menu(player);
        // call do menu in here on room
        // check if room and or player has items and all that
    }
}
