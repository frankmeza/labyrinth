use crate::{constants, menu};

#[derive(Debug)]
pub struct Player {
    pub torch_lit: bool,
    pub current_room: String,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new() -> Self {
        let inventory = vec![String::from(constants::MATCHES)];

        Player {
            torch_lit: true,
            current_room: String::from(constants::STARTING_ROOM),
            inventory,
        }
    }

    pub fn set_torch_lit(&mut self, is_lit: bool) {
        *self = Self {
            torch_lit: is_lit,
            inventory: self.inventory.clone(),
            current_room: String::from(&self.current_room),
        };
    }

    pub fn set_current_room(&mut self, room_name: &str) {
        *self = Player {
            current_room: String::from(room_name),
            inventory: self.inventory.clone(),
            torch_lit: self.get_torch_lit(),
        };
    }

    pub fn pick_up_item(&mut self, item_name: &str) {
        self.inventory.push(String::from(item_name));
    }

    pub fn drop_item(&mut self, name: &str) {
        let found_item = self.inventory.iter().position(|i| i == name);
        let index = found_item.unwrap(); // TODO handle better

        self.inventory.remove(index);

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

    pub fn handle_player_torch(&mut self) {
        if !self.get_torch_lit() && self.has_item("matches") {
            self.set_torch_lit(true)
        }
    }
}
