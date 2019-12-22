use crate::{
    ascii, constants,
    item::Item,
    space::{EmptySpace, SpaceType},
    story,
};

pub struct Player {
    pub torch_lit: bool,
    pub current_room: SpaceType,
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn new() -> Self {
        let starting_room =
            SpaceType::Empty(EmptySpace::new(String::from(constants::STARTING_ROOM)));

        let matches = Item::new(
            String::from("matches"),
            String::from("box of matches"),
            ascii::matches(),
        );

        Player {
            torch_lit: true,
            current_room: starting_room,
            inventory: vec![matches],
        }
    }

    pub fn get_torch_lit(&self) -> bool {
        self.torch_lit
    }

    pub fn has_items(&self) -> bool {
        self.inventory.len() > 0
    }

    pub fn handle_player_has_items() {
        println!("{}", story::view_items());
        println!("{}", story::drop_item());
    }
}
