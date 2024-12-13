#![allow(unused)]

use std::{os::windows::thread, vec};

use rand::{self, random, rngs::ThreadRng};

use crate::player::Player;

type Bullet = bool;

struct Game<P1: Player, P2: Player> {
    rand: ThreadRng,
    bullets: Vec<Bullet>,
    player1: P1,
    player2: P2,
}

impl<P1: Player, P2: Player> Game<P1, P2> {
    fn new_game() -> Self {
        Self {
            rand: rand::thread_rng(),
            bullets: Vec::new(),
            player1: new(),
            player2: new(),
        }
    }
}
