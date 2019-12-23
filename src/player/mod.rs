use crate::{ascii, constants, item::Item, menu};
#[derive(Debug)]
pub struct Player {
    pub torch_lit: bool,
    pub current_room: String,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new() -> Self {
        let matches = Item::new(
            String::from("matches"),
            String::from("box of matches"),
            ascii::matches(),
        );

        Player {
            torch_lit: true,
            current_room: String::from(constants::STARTING_ROOM),
            inventory: vec![matches.get_name()],
        }
    }

    pub fn set_current_room(&mut self, room_name: &str) {
        *self = Player {
            current_room: String::from(room_name),
            inventory: self.inventory.clone(),
            torch_lit: self.get_torch_lit(),
        };
    }

    pub fn get_torch_lit(&self) -> bool {
        self.torch_lit
    }

    pub fn has_items(&self) -> bool {
        self.inventory.len() > 0
    }

    pub fn has_item(&self, item_name: &str) -> bool {
        let mut iter = self.inventory.iter();
        let has_item = &iter.find(|&item| item == item_name);

        match has_item {
            None => false,
            Some(_item) => true,
        }
    }

    pub fn handle_player_has_items() {
        println!("{}", menu::view_items());
        println!("{}", menu::drop_item());
    }
}
