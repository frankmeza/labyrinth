use crate::player::Player;
use crate::space::Space;

pub struct MinotaurSpace {
    description: String,
}

impl Space for MinotaurSpace {
    fn do_menu(_player: &Player) -> bool {
        true
    }
}
