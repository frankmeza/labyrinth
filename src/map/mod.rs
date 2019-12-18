use crate::space::{EmptySpace, ItemSpace, MinotaurSpace};

enum SpaceType {
    EmptySpace,
    ItemSpace,
    MinotaurSpace,
}

pub struct Map {
    spaces: Vec<SpaceType>,
}
