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

pub fn get_exit_options(option: &u8) -> String {
    match option {
        0 => String::from("enter 1 to exit to the top"),
        1 => String::from("enter 2 to exit to the right"),
        2 => String::from("enter 3 to exit to the bottom"),
        3 => String::from("enter 4 to exit to the left"),
        _ => String::from(""),
    }
}
