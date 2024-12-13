pub enum Item {
    Knife,
    Handcuffs,
    Magnifier,
    Cigarettes,
    Inverter,
    Beer,
}

pub enum ItemResponse {
    // Take whatever you think is neccessary or add new
    None,
    NextBullet(bool),
    AnyBullet(bool),
    SkipTurns(u32),
    Heal(u32),
    Damage(u32),
}

pub enum Effect {
    Handcuffed(u32),
    PowerShot,
}
