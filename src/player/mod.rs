use crate::{
    constants as c,
    map::Map,
    space::{EmptySpace, SpaceType},
};

#[derive(Debug)]
pub struct Player {
    pub torch_lit: bool,
    pub current_room: String,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new() -> Self {
        let inventory = vec![String::from(c::MATCHES)];

        Player {
            torch_lit: true,
            current_room: String::from(c::STARTING_ROOM),
            inventory,
        }
    }

    // GETTERS //

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

    pub fn get_torch_lit(&self) -> bool {
        self.torch_lit
    }

    pub fn get_items(&self) -> &Vec<String> {
        &self.inventory
    }

    pub fn get_current_room(&self) -> String {
        String::from(&self.current_room)
    }

    // SETTERS //

    pub fn set_torch_lit(&mut self, is_lit: bool) {
        *self = Self {
            torch_lit: is_lit,
            inventory: self.inventory.clone(), // todo come back to this another way
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

    pub fn take_item_from_space(&mut self, item_name: &str, index_of_space: usize) -> Map {
        let mut map = Map::new();
        let space_type = map.get_space(index_of_space);

        let space_without_items = EmptySpace::new(String::from(&space_type.get_room_name()));

        let updated_space_type = SpaceType::Empty(space_without_items);
        map.spaces[index_of_space] = updated_space_type;

        self.inventory.push(String::from(item_name));
        map
    }

    pub fn drop_item(&mut self, name: &str) {
        let found_item = self.inventory.iter().position(|i| i == name);

        match found_item {
            None => println!("drop_item is very virus"),
            Some(index) => {
                let _removed = self.inventory.remove(index);
            }
        }
    }

    pub fn handle_player_torch(&mut self) {
        if !self.get_torch_lit() && self.has_item("matches") {
            self.set_torch_lit(true)
        }
    }
}
