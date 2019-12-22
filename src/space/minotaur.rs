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

// impl Room for MinotaurSpace {
//     fn do_menu(&self, _player: &Player) -> bool {
//         let s = self;
//         true
//     }
// }
