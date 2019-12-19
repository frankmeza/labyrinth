pub fn handle_choice(choice: &str, is_valid_choice: &mut bool) -> bool {
    match choice {
        "1" => {
            println!("entering game");
            *is_valid_choice = true;
            true
        }
        "2" => {
            println!("quitting game");
            *is_valid_choice = true;
            false
        }
        _ => {
            println!("invalid");
            false
        }
    }
}
