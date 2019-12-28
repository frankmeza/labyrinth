use crate::space::Space;

#[derive(Debug)]
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
