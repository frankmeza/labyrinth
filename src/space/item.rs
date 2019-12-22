use crate::space::Space;

pub struct ItemSpace {
    pub space: Space,
}

impl ItemSpace {
    pub fn new(description: String) -> Self {
        ItemSpace {
            space: Space::new(String::from(&description)),
        }
    }
}
