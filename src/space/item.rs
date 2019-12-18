use crate::player::Player;
use crate::space::Space;

pub struct ItemSpace {
    description: String,
}

impl Space for ItemSpace {
    fn do_menu(_player: &Player) -> bool {
        true
    }
}
