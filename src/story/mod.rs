pub fn star_separator() -> String {
    vec![
        "",
        "*****************************************************************************",
        "*****************************************************************************",
        "",
    ]
    .join("\n")
}

pub fn lost_in_a_labyrinth() -> String {
    vec![
        "      LOST IN A LABYRINTH...",
        "",
        "you find yourself inside of a huge labyrinth",
        "the rooms are dark and damp and made of solid stone",
        "the only way out is through...",
        "",
        "you are armed only with a torch, and matches to relight the torch",
        "you must enter the labyrinth and fight the minotaur to escape",
        "there may be items you encounter along the way that are necessary",
        "or required to win this fight",
        "",
        "choose your path wisely, or you may end up lost in the labyrinth forever...",
        "",
    ]
    .join("\n")
}

pub fn items_on_ground() -> String {
    String::from("there are items on the ground")
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
