use crate::player::Player;
use crate::space::{self, Space};
use std::collections::HashMap;

pub struct MinotaurSpace {
    pub description: String,
    pub exits: HashMap<usize, usize>,
}

impl Space for MinotaurSpace {
    fn new(description: String) -> Self {
        let exits = space::exits(&description);

        MinotaurSpace { description, exits }
    }

    fn has_items() -> bool {
        false
    }

    fn do_menu(_player: &Player) -> bool {
        true
    }
}
