pub trait Item {
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
        self.name
    }

    pub fn get_description(&self) -> String {
        self.description
    }

    pub fn get_art(&self) -> String {
        self.art
    }
}
