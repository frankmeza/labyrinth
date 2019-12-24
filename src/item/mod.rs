#[derive(Debug)]
pub struct Item {
    name: String,
    description: String,
    art: String,
}

impl Item {
    pub fn new(
        name: String,
        description: String,
        art: String,
    ) -> Self {
        Item { name, description, art }
    }

    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }

    pub fn get_description(&self) -> String {
        String::from(&self.description)
    }

    pub fn get_art(&self) -> String {
        String::from(&self.art)
    }
}
