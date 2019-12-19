use crate::player::Player;
use crate::space::{self, Space};
use std::collections::HashMap;

pub struct ItemSpace {
    pub description: String,
    pub exits: HashMap<u8, u8>,
}

impl Space for ItemSpace {
    fn new(description: String) -> Self {
        let exits = space::exits(&description);

        ItemSpace { description, exits }
    }

    fn has_items() -> bool {
        // todo
        true
    }

    fn do_menu(_player: &Player) -> bool {
        true
    }
}
