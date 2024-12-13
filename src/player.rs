use crate::items::Item;


pub trait Player {
    fn new() -> Self;
    fn take_turn(&mut self) -> PlayerChoice;
    fn take_items(&mut self, items: Vec<Item>);
    fn take_damage(&mut self, damage: u32);
    fn reset_health(&mut self, health: u32);
}

pub enum PlayerChoice {
    ShootSelf,
    ShootOther,
}

pub struct DefaultPlayer {
    inventory: [Option<Item>; 8],
    health: u32,
}

impl Player for DefaultPlayer {
    fn take_turn(&mut self) -> PlayerChoice {
        PlayerChoice::ShootOther
    }

    fn take_items(&mut self, items: Vec<Item>) {
        //self.inventory
    }

    fn reset_health(&mut self, health: u32) {
        self.health = health
    }

    fn take_damage(&mut self, damage: u32) {
        self.health -= damage
    }

    fn new() -> Self {
        Self {
            inventory: [None; 8],
            health: 0,
        }
    }
}
