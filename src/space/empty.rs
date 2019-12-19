use crate::ascii;
use crate::player::Player;
use crate::space::{self, Space};
use std::{collections::HashMap, io};

pub struct EmptySpace {
    pub description: String,
    pub exits: HashMap<usize, usize>,
}

impl Space for EmptySpace {
    fn new(description: String) -> Self {
        let exits = space::exits(&description);

        EmptySpace { description, exits }
    }

    fn has_items() -> bool {
        false
    }

    fn do_menu(player: &Player) -> bool {
        let mut got_input = false;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap().to_string();

        while !got_input {
            if player.get_torch_lit() {
                println!("{}", ascii::lit_torch());

                // if EmptySpace::has_items() is false, right?

                // now get the map to know the exits for this room
            }
        }

        true
    }
}

impl EmptySpace {}
