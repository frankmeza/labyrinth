fn show_options() {
    let display = vec![
        "______________________________________________",
        "-------------------- MENU --------------------",
        "   1 --- play game",
        "   2 --- quit game",
        "______________________________________________",
    ];

    for line in &display {
        println!("{}", line);
    }
}

pub fn display() {
    show_options();
}