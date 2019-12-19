pub struct Player {
    pub torch_lit: bool,
}

impl Player {
    pub fn new() -> Self {
        let mut player = Player {
            torch_lit: false,
        };

        player
    }

    pub fn get_torch_lit(&self) -> bool {
        self.torch_lit
    }
}