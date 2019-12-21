use crate::ascii;
use crate::player::Player;
use crate::space::{Room, Space};
use std::io;

pub struct EmptySpace {
    pub space: Space,
}

impl EmptySpace {
    pub fn new(description: String) -> Self {
        EmptySpace {
            space: Space::new(String::from(&description)),
        }
    }
}

impl Room for EmptySpace {
    fn do_menu(&self, player: &Player) -> bool {
        let mut got_input = false;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap().to_string();

        while !got_input {
            if player.get_torch_lit() {
                println!("{}", ascii::lit_torch());

                if self.space.has_items() {

                }

                // now get the map to know the exits for this room
            }
        }

        true
    }
}

impl EmptySpace {}
