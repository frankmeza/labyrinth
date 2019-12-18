use crate::player::Player;

mod empty;
mod item;
mod minotaur;

pub use empty::EmptySpace;
pub use item::ItemSpace;
pub use minotaur::MinotaurSpace;

pub trait Space {
    fn do_menu(player: &Player) -> bool;
}
