fn show_options() -> String {
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

pub fn display() {
    println!("{}", show_options());
}
