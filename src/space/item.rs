use crate::{
    player::Player,
    space::{Room, Space},
};

pub struct ItemSpace {
    pub space: Space,
}

impl ItemSpace {
    pub fn new(description: String) -> Self {
        ItemSpace {
            space: Space::new(String::from(&description)),
        }
    }
}

impl Room for ItemSpace {
    // todo
    fn has_items() -> bool {
        true
    }

    fn do_menu(_player: &Player) -> bool {
        true
    }
}
