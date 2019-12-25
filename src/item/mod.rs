use crate::{ascii, constants};
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
            String::from(constants::MATCHES),
            Item::new(
                String::from(constants::MATCHES),
                String::from(constants::MATCHES_DESC),
                ascii::matches(),
            ),
        );

        items.insert(
            String::from(constants::ARROWS),
            Item::new(
                String::from(constants::ARROWS),
                String::from(constants::ARROWS_DESC),
                ascii::arrows(),
            ),
        );

        items.insert(
            String::from(constants::BOW),
            Item::new(
                String::from(constants::BOW),
                String::from(constants::BOW_DESC),
                ascii::bow(),
            ),
        );

        items.insert(
            String::from(constants::SHIELD),
            Item::new(
                String::from(constants::SHIELD),
                String::from(constants::SHIELD_DESC),
                ascii::shield(),
            ),
        );

        items.insert(
            String::from(constants::HEALTH_POTION),
            Item::new(
                String::from(constants::HEALTH_POTION),
                String::from(constants::HEALTH_POTION_DESC),
                ascii::health_potion(),
            ),
        );

        items
    }

}