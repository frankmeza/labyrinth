use crate::space::Space;

pub struct MinotaurSpace {
    pub space: Space,
}

impl MinotaurSpace {
    pub fn new(description: String) -> Self {
        MinotaurSpace {
            space: Space::new(String::from(&description)),
        }
    }
}
