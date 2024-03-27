use crate::Item;

pub struct PlayerData {
    pub inventory: Vec<Item>,
    pub storage: Vec<Item>,
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData {
            inventory: Vec::new(),
            storage: Vec::new(),
        }
    }

    pub fn unload(&mut self) {
        self.inventory.clear();
        self.storage.clear();
    }
}
