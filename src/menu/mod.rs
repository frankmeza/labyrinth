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

pub fn cannot_relight_torch() -> String {
    vec![
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "without a lit torch you can't do anything and now you lose",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
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
