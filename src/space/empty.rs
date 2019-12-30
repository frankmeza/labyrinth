use crate::space::Space;

#[derive(Debug)]
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
