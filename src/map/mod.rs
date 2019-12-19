use crate::space::{EmptySpace, ItemSpace, MinotaurSpace, Space};

pub enum SpaceType {
    EmptySpace(EmptySpace),
    ItemSpace(ItemSpace),
    MinotaurSpace(MinotaurSpace),
}

pub struct Map {
    pub spaces: Vec<SpaceType>,
}

fn generate_spaces() -> Vec<SpaceType> {
    vec![
        SpaceType::EmptySpace(EmptySpace::new("Starting Room".to_string())),
        SpaceType::ItemSpace(ItemSpace::new("Room 1".to_string())),
        SpaceType::EmptySpace(EmptySpace::new("Room 2".to_string())),
        SpaceType::ItemSpace(ItemSpace::new("Room 3".to_string())),
        SpaceType::ItemSpace(ItemSpace::new("Room 4".to_string())),
        SpaceType::EmptySpace(EmptySpace::new("Room 5".to_string())),
        SpaceType::ItemSpace(ItemSpace::new("Room 6".to_string())),
        SpaceType::MinotaurSpace(MinotaurSpace::new("Final Room".to_string())),
    ]
}

impl Map {
    pub fn new() -> Self {
        let spaces = generate_spaces();

        Map { spaces }
    }

    pub fn get_space(&self, index: usize) -> &SpaceType {
        &self.spaces[index]
    }
}
