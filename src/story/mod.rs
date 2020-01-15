pub fn star_separator() -> String {
    vec![
        "",
        "*****************************************************************************",
        "*****************************************************************************",
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
    String::from(
        "there are items on the ground\nit really seems like something from this room \
         is going to be very important",
    )
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

pub fn player_currently_carrying() -> String {
    String::from("you are currently carrying:")
}

pub fn what_player_can_drop() -> String {
    String::from("what are you going to drop?")
}

pub fn ready_to_fight() -> String {
    vec![
        "you have collected the shield, bow, and arrows...",
        "you are ready to take on the minotaur",
    ]
    .join("\n")
}

pub fn you_have_health_potion() -> String {
    vec![
        "you have a bottle of health potion in your inventory",
        "this gives you the ability to revive yourself and",
        "continue fighting the first time you are injured",
    ]
    .join("\n")
}

pub fn enter_minotaur_lair() -> String {
    vec![
        "you arm yourself with your weapons and enter the minotaur's lair",
        "hopefully you make it through this alive...",
        "** at start of fight player health is 20",
        "** at start of fight minotaur health is 20",
    ]
    .join("\n")
}

// pub fn print_fighters_initial_health() -> String {}

pub fn you_killed_minotaur() -> String {
    vec![
        "****************************************************************",
        "you killed the minotaur and get to escape the labyrinth!",
        "****************************************************************",
    ]
    .join("\n")
}

pub fn minotaur_killed_you() -> String {
    vec![
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "the minotaur killed you, your soul is stuck here forever",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    ]
    .join("\n")
}
