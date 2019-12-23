use crate::{constants, map::Map, player::Player};

pub fn show_options() -> String {
    vec![
        "______________________________________________",
        "-------------------- MENU --------------------",
        "   1 --- play game",
        "   2 --- quit game",
        "______________________________________________",
    ]
    .join("\n")
}

pub fn can_relight_torch() -> String {
    vec![
        "good thing you have some matches!",
        "enter 5 to relight your torch",
    ]
    .join("\n")
}

pub fn get_exit_options(option: &usize) -> String {
    match option {
        0 => String::from("enter 1 to exit to the top"),
        1 => String::from("enter 2 to exit to the right"),
        2 => String::from("enter 3 to exit to the bottom"),
        3 => String::from("enter 4 to exit to the left"),
        _ => String::from(""),
    }
}

pub fn pick_up_items() -> String {
    String::from("enter 0 to pick up the items")
}

pub fn view_items() -> String {
    String::from("enter i to view your items")
}

pub fn drop_item() -> String {
    String::from("enter d to drop an item")
}

pub fn quit_game() -> String {
    String::from("enter q to quit game")
}

pub fn handle_move_to_room(input: &str, player: &mut Player) -> bool {
    let map = Map::new();
    let mut is_moving_to_room = true;

    // .trim() is necessary!
    // see comment #1 at bottom
    let space_type = match input.trim() {
        constants::CHOICE_1 => map.get_space(0),
        constants::CHOICE_2 => map.get_space(1),
        constants::CHOICE_3 => map.get_space(2),
        constants::CHOICE_4 => map.get_space(3),
        _ => {
            is_moving_to_room = false;
            map.get_space(0)
        }
    };

    if !is_moving_to_room {
        match input {
            constants::CHOICE_0 => {
                // TODO room:: if (has_items())
            }
            constants::CHOICE_5 => {}
            constants::CHOICE_I => {
                // TODO if (player->has_items())
                // print player inventory
            }
            constants::CHOICE_D => {
                // TODO drop player item
            }
            constants::CHOICE_Q => return false,
            _ => return false,
        }
    }

    let room_name = space_type.get_room_name();
    player.set_current_room(&room_name);

    true
}

// #1: http://danielnill.com/rust_tip_compairing_strings