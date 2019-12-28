use crate::{ascii, constants as c};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Item {
    name: String,
    description: String,
    art: String,
}

impl Item {
    pub fn new(name: String, description: String, art: String) -> Self {
        Item {
            name,
            description,
            art,
        }
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

    pub fn all_items() -> HashMap<String, Item> {
        let mut items = HashMap::new();

        items.insert(
            String::from(c::MATCHES),
            Item::new(
                String::from(c::MATCHES),
                String::from(c::MATCHES_DESC),
                ascii::matches(),
            ),
        );

        items.insert(
            String::from(c::ARROWS),
            Item::new(
                String::from(c::ARROWS),
                String::from(c::ARROWS_DESC),
                ascii::arrows(),
            ),
        );

        items.insert(
            String::from(c::BOW),
            Item::new(
                String::from(c::BOW),
                String::from(c::BOW_DESC),
                ascii::bow(),
            ),
        );

        items.insert(
            String::from(c::SHIELD),
            Item::new(
                String::from(c::SHIELD),
                String::from(c::SHIELD_DESC),
                ascii::shield(),
            ),
        );

        items.insert(
            String::from(c::HEALTH_POTION),
            Item::new(
                String::from(c::HEALTH_POTION),
                String::from(c::HEALTH_POTION_DESC),
                ascii::health_potion(),
            ),
        );

        items
    }
}
