use crate::ascii;
use crate::space::{self, EmptySpace, ItemSpace, MinotaurSpace, Space};
use std::collections::HashMap;

pub enum SpaceType {
    EmptySpace(EmptySpace),
    ItemSpace(ItemSpace),
    MinotaurSpace(MinotaurSpace),
}

impl SpaceType {
    fn get_space_exits(&self) -> &HashMap<usize, usize> {
        match self {
            SpaceType::EmptySpace(space) => &space.exits,
            SpaceType::ItemSpace(space) => &space.exits,
            SpaceType::MinotaurSpace(space) => &space.exits,
        }
    }

    fn get_room_name(&self) -> String {
        match self {
            SpaceType::EmptySpace(space) => String::from(&space.description),
            SpaceType::ItemSpace(space) => String::from(&space.description),
            SpaceType::MinotaurSpace(space) => String::from(&space.description),
        }
    }
}

pub struct Map {
    pub spaces: Vec<SpaceType>,
}

fn generate_spaces() -> Vec<SpaceType> {
    vec![
        SpaceType::EmptySpace(EmptySpace::new(String::from("Starting Room"))),
        SpaceType::ItemSpace(ItemSpace::new(String::from("Room 1"))),
        SpaceType::EmptySpace(EmptySpace::new(String::from("Room 2"))),
        SpaceType::ItemSpace(ItemSpace::new(String::from("Room 3"))),
        SpaceType::ItemSpace(ItemSpace::new(String::from("Room 4"))),
        SpaceType::EmptySpace(EmptySpace::new(String::from("Room 5"))),
        SpaceType::ItemSpace(ItemSpace::new(String::from("Room 6"))),
        SpaceType::MinotaurSpace(MinotaurSpace::new(String::from("Final Room"))),
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
