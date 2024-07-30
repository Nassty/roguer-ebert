use std::collections::HashMap;

use pathfinding::prelude::astar;
use rand::Rng;
use symmetric_shadowcasting::Pos;

use crate::{distance, player::Player, Block};

#[derive(Debug, PartialEq, Clone)]
pub struct Enemy {
    pub hp: i32,
    luck: i32,
    timer: i32,
    pub pos: Pos,
}

impl Enemy {
    pub fn new(hp: i32, pos: Pos) -> Self {
        let luck = rand::thread_rng().gen_range(0..5);
        Self {
            hp,
            pos,
            luck,
            timer: 0,
        }
    }
    pub fn hit_by(&mut self, damage: i32) {
        self.hp -= damage;
    }
    pub fn update(
        &mut self,
        player: &mut Player,
        map: &HashMap<Pos, Block>,
        enemies: &HashMap<Pos, Self>,
        flag: bool,
    ) {
        if !flag {
            return;
        }
        if self.timer < 1 {
            self.timer = 22;
        }
        self.timer -= 1;
        if self.timer % 3 == 0 && distance(self.pos, player.pos) < 3.0 {
            let damage = rand::thread_rng().gen_range(0..self.luck + 1);
            player.hit_by(damage);
        }
        let path = astar(
            &self.pos,
            |&(x, y)| {
                vec![
                    (x + 1, y + 2),
                    (x + 1, y - 2),
                    (x - 1, y + 2),
                    (x - 1, y - 2),
                    (x + 2, y + 1),
                    (x + 2, y - 1),
                    (x - 2, y + 1),
                    (x - 2, y - 1),
                ]
                .into_iter()
                .filter(|x| !map.contains_key(x) && !enemies.contains_key(x) && self.timer % 5 == 0)
                .map(|p| (p, 1))
            },
            |&(x, y)| (player.pos.0.abs_diff(x) + player.pos.1.abs_diff(y)) / 2,
            |&p| p == player.pos,
        );
        if let Some((p, _)) = &path {
            if let Some(x) = p.get(1) {
                if *x != player.pos {
                    self.pos = *x
                }
            }
        }
    }
}
