use crate::items::{Effect, Item};

pub const MAX_ITEMS: usize = 8;

#[derive(Default)]
pub struct Player {
    pub items: [Option<Item>; MAX_ITEMS],
    pub hp: u32,
    pub effects: Vec<Effect>,
}
