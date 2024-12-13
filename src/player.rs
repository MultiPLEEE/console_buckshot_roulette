use crate::items::Item;

pub const MAX_ITEMS: usize = 8;
pub struct Player {
    pub items: [Option<Item>; MAX_ITEMS],
    pub hp: u32,
}