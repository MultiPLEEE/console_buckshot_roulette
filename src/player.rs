
pub trait Player {
    fn take_turn(&mut self) -> PlayerChoice;
    fn take_items(&mut self, items: Vec<Item>);
    fn take_damage(&mut self, damage: u32);
    fn reset_health(&mut self, health: u32);
}

enum PlayerChoice {
    ShootSelf,
    ShootOther
}

pub struct DefaultPlayer {
    inventory: Vec<Item>,
    health: u32,
}

impl Player for DefaultPlayer {
    fn take_turn(&mut self) -> PlayerChoice {
        
    }

    fn take_items(&mut self, items: Vec<Item>) {
        todo!()
    }

    fn reset_health(&mut self, health: u32) {
        self.health = health
    }

    fn take_damage(&mut self, damage: u32) {
        todo!()
    }
}