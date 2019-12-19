use crate::{
    ascii,
    space::{self, EmptySpace, ItemSpace, MinotaurSpace},
};
use std::collections::HashMap;

pub enum SpaceType {
    Empty(EmptySpace),
    Item(ItemSpace),
    Minotaur(MinotaurSpace),
}

impl SpaceType {
    fn get_space_exits(&self) -> &HashMap<usize, usize> {
        match self {
            SpaceType::Empty(e) => &e.space.exits,
            SpaceType::Item(i) => &i.space.exits,
            SpaceType::Minotaur(m) => &m.space.exits,
        }
    }

    fn get_room_name(&self) -> String {
        match self {
            SpaceType::Empty(e) => String::from(&e.space.description),
            SpaceType::Item(i) => String::from(&i.space.description),
            SpaceType::Minotaur(m) => String::from(&m.space.description),
        }
    }
}

pub struct Map {
    pub spaces: Vec<SpaceType>,
}

fn generate_spaces() -> Vec<SpaceType> {
    vec![
        SpaceType::Empty(EmptySpace::new(String::from("Starting Room"))),
        SpaceType::Item(ItemSpace::new(String::from("Room 1"))),
        SpaceType::Empty(EmptySpace::new(String::from("Room 2"))),
        SpaceType::Item(ItemSpace::new(String::from("Room 3"))),
        SpaceType::Item(ItemSpace::new(String::from("Room 4"))),
        SpaceType::Empty(EmptySpace::new(String::from("Room 5"))),
        SpaceType::Item(ItemSpace::new(String::from("Room 6"))),
        SpaceType::Minotaur(MinotaurSpace::new(String::from("Final Room"))),
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

    pub fn enter_labyrinth(&self) {
        let starting_room = self.get_space(0);
        let exits = starting_room.get_space_exits();

        println!("{}", ascii::left_forward_right_room());
        println!("{}\n\n", &starting_room.get_room_name());

        println!("{}\n", ascii::lit_torch());
        println!("{}", space::get_exit_options(exits));
    }
}
