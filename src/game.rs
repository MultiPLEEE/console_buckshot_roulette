#![allow(unused)]

use std::{os::windows::thread, vec};

use rand::{self, random, rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::{
    interface::{self, Interface},
    player::Player,
};

type Bullet = bool;

enum Turn<I1, I2> {
    Player1(I1),
    Player2(I2),
}
s
// impl Turn {
//     fn switch(&mut self) {
//         match self {
//             Turn::Player1 => *self = Turn::Player2,
//             Turn::Player2 => *self = Turn::Player1,
//         }
//     }
// }

struct Game<I1: Interface, I2: Interface> {
    rand: ThreadRng,
    bullets: Vec<Bullet>,
    turn: Turn<I1, I2>,
    player1: Player,
    player2: Player,
    interface1: I1,
    interface2: I2,
}

impl<I1: Interface, I2: Interface> Game<I1, I2> {
    fn new_game(interface1: I1, interface2: I2) -> Self {
        let player1 = Default::default();

        let player2 = Default::default();

        Self {
            rand: rand::thread_rng(),
            bullets: Vec::new(),
            turn: Turn::Player1(interface1),
            player1,
            player2,
            interface1,
            interface2,
        }
    }

    fn start(&mut self) {
        let total_bullets: usize = self.rand.gen_range(3..=8);
        let red_bullets = self.rand.gen_range(1..total_bullets);

        let mut vector_bullets = Vec::new();
        vector_bullets.append(&mut vec![true; red_bullets]);
        vector_bullets.append(&mut vec![false; total_bullets - red_bullets]);
        vector_bullets.shuffle(&mut self.rand);

        self.bullets = vector_bullets;

        self.player1.hp = 4;
        self.player1.effects = Vec::new();

        self.player2.hp = 4;
        self.player2.effects = Vec::new();
    }

    fn turn(&mut self) {
        let (offender, defender) = match self.turn {
            Turn::Player1 => (&mut self.player1, &mut self.p\\layer2),
            Turn::Player2 => (&mut self.player2, &mut self.player1),
        };
    }
}
