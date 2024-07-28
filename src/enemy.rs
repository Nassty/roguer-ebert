use std::collections::HashMap;

use pathfinding::prelude::astar;
use symmetric_shadowcasting::Pos;

use crate::Block;

#[derive(Debug, PartialEq, Clone)]
pub struct Enemy {
    hp: u32,
    pub pos: Pos,
}

impl Enemy {
    pub fn new(hp: u32, pos: Pos) -> Self {
        Self { hp, pos }
    }
    pub fn update(
        &mut self,
        center: Pos,
        map: &HashMap<Pos, Block>,
        enemies: &HashMap<Pos, Self>,
        flag: bool,
    ) {
        if !flag || flag {
            return;
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
                .filter(|x| !map.contains_key(x))
                .map(|p| (p, 1))
            },
            |&(x, y)| (center.0.abs_diff(x) + center.1.abs_diff(y)) / 3,
            |&p| p == center,
        );
        if let Some((p, _)) = path {
            if let Some(x) = p.get(1) {
                if !enemies.contains_key(x) && *x != center {
                    self.pos = *x
                }
            }
        }
    }
}
