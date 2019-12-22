use crate::space::Space;

pub struct EmptySpace {
    pub space: Space,
}

impl EmptySpace {
    pub fn new(description: String) -> Self {
        EmptySpace {
            space: Space::new(String::from(&description)),
        }
    }
}
