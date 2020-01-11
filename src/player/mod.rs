use crate::constants as c;

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
        self.torch_lit = is_lit;
    }

    pub fn set_current_room(&mut self, room_name: &str) {
        self.current_room = String::from(room_name);
    }

    pub fn add_item(&mut self, item_name: &str) {
        self.inventory.push(String::from(item_name));
    }

    // TODO
    // pub fn drop_item(&mut self, name: &str) {
    //     let found_item = self.inventory.iter().position(|i| i == name);

    //     match found_item {
    //         None => println!("drop_item is very virus"),
    //         Some(index) => {
    //             let _removed = self.inventory.remove(index);
    //         }
    //     }
    // }

    pub fn handle_player_torch(&mut self) {
        if !self.get_torch_lit() && self.has_item("matches") {
            self.set_torch_lit(true)
        }
    }
}
