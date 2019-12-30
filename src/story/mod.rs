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

pub fn cannot_relight_torch() -> String {
    vec![
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "without a lit torch you can't do anything and now you lose",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    ]
    .join("\n")
}

pub fn player_cannot_pick_up_item() -> String {
    vec![
        "you don't have any more room in your inventory!",
        "you might need to leave something behind",
    ]
    .join("\n")
}

pub fn all_items_picked_up() -> String {
    String::from("you picked up all the items")
}
