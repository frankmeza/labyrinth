use crate::player::Player;
use crate::space::{Room, Space};

pub struct MinotaurSpace {
    pub space: Space,
}

impl MinotaurSpace {
    pub fn new(description: String) -> Self {
        MinotaurSpace {
            space: Space::new(String::from(&description)),
        }
    }
}

impl Room for MinotaurSpace {
    // todo
    fn has_items() -> bool {
        false
    }

    fn do_menu(_player: &Player) -> bool {
        true
    }
}
