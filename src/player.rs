use crate::items::{Item, Effect};

pub const MAX_ITEMS: usize = 8;
pub struct Player {
    pub items: [Option<Item>; MAX_ITEMS],
    pub hp: u32,
    pub effects: Vec<Effect>
}