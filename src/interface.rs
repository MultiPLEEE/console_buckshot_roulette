use crate::{items::{Item, ItemResponse}, player::MAX_ITEMS};

pub trait Interface {
    fn take_turn(info: TurnInfo) -> PlayerChoice;
    fn start_info(info: StartInfo);
    fn item_response(info: ItemResponse);
}

type ItemIndex = usize;
pub enum PlayerChoice {
    ShootSelf,
    ShootOther,
    UseItem(ItemIndex),
}

pub struct TurnInfo<'a> {
    pub self_hp: u32,
    pub other_hp: u32,
    pub self_items: &'a [Option<Item>; MAX_ITEMS],
    pub other_items: &'a [Option<Item>; MAX_ITEMS],
}

pub struct StartInfo {
    pub bullets: Vec<bool>,
}
