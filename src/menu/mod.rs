use crate::{item::Item, story};

pub fn can_relight_torch() -> String {
    vec![
        "good thing you have some matches!",
        "enter 5 to relight your torch",
    ]
    .join("\n")
}

fn drop_item() -> String {
    String::from("enter d to drop an item")
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

pub fn player_has_items() {
    println!("{}", view_items());
    println!("{}", drop_item());
}

pub fn quit_game() -> String {
    String::from("enter q to quit game")
}

pub fn show_options() -> String {
    vec![
        "______________________________________________",
        "-------------------- MENU --------------------",
        "   1 --- play game",
        "   2 --- quit game",
        "______________________________________________",
        "make selection:",
    ]
    .join("\n")
}

pub fn print_space_items(items: &Vec<String>) {
    if items.len() > 0 {
        println!("{}", story::items_on_ground());
    }

    Item::print_owned_items(&items);

    println!("{}", pick_up_items());
}

pub fn print_player_items(items: &Vec<String>) {
    Item::print_owned_items(&items);
}

fn view_items() -> String {
    String::from("enter i to view your items")
}

pub fn cancel_drop_item() -> String {
    String::from("enter x to cancel dropping")
}

pub fn print_items_to_drop(items: &Vec<String>) {
    let items_map = Item::all_items();
    let mut counter = 0;

    for name in items.iter() {
        counter += 1;
        let found_item = items_map.get(name);

        match found_item {
            None => println!("print_items_to_drop is very virus"),
            Some(item) => {
                println!("enter {} to drop {}", counter, item.get_description());
                println!("{}\n", item.get_art());
            }
        }
    }
}
