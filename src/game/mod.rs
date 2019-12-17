fn print_start_message() {
    let display = vec![
        "*****************************************************************************",
        "*****************************************************************************",
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
        "     .   .",
        "    /)  /)",
        ".-.((,~,)).-,",
        "`-,  ",
        "   \\",
        "   /  ,-'",
        "  ( @   @ )",
        "   ",
        "     \\",
        "     /",
        "    -o.o-",
        "    () ()",
        "   ()()()()",
        "",
        "*****************************************************************************",
        "*****************************************************************************",
        "",
    ];

    for line in &display {
        println!("{}", line);
    }
}

pub fn run() {
    print_start_message();
}