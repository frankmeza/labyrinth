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

pub fn display() {
    println!("{}", show_options());
}
