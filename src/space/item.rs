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

// impl Room for ItemSpace {
//     fn do_menu(&self, _player: &Player) -> bool {
//         let s = self;
//         true
//     }
// }
