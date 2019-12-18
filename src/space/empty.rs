use crate::player::Player;
use crate::space::{self, Space};
use std::collections::HashMap;

pub struct EmptySpace {
    pub description: String,
    pub exits: HashMap<u8, u8>,
}

impl Space for EmptySpace {
    fn new(description: String) -> Self {
        let exits = space::exits(&description);

        EmptySpace { description, exits }
    }

    fn do_menu(_player: &Player) -> bool {
        true
    }
}
