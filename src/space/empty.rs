use crate::player::Player;
use crate::space::Space;

pub struct EmptySpace {
    description: String,
}

impl Space for EmptySpace {
    fn do_menu(_player: &Player) -> bool {
        true
    }
}