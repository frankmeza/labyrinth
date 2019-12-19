use crate::constants;

pub fn handle_choice(choice: &str, is_valid_choice: &mut bool) -> bool {
    match choice {
        constants::CHOICE_1 => {
            println!("{}", constants::ENTER);
            *is_valid_choice = true;
            true
        }
        constants::CHOICE_2 => {
            println!("{}", constants::QUIT);
            *is_valid_choice = true;
            false
        }
        _ => {
            println!("{}", constants::INVALID);
            false
        }
    }
}
