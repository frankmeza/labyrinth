use crate::constants as c;

pub fn handle_choice(choice: &str, is_valid_choice: &mut bool) -> bool {
    match choice {
        c::CHOICE_1 => {
            println!("{}", c::ENTER);
            *is_valid_choice = true;
            true
        }
        c::CHOICE_2 => {
            println!("{}", c::QUIT);
            *is_valid_choice = true;
            false
        }
        _ => {
            println!("{}", c::INVALID);
            false
        }
    }
}
