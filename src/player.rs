use crate::utils::Pos;
use rand::Rng;

use crate::{distance, enemy::Enemy};

#[derive(Debug)]
pub enum PlayerState {
    Walking,
    Combat(Vec<Pos>),
}

#[derive(Debug)]
pub struct Player {
    pub hp: i32,
    ac: i32,
    luck: i32,
    pub pos: Pos,
    pub state: PlayerState,
    pub items: Vec<String>,
    pub carrying: String,
}

impl Player {
    pub fn new(pos: Pos) -> Self {
        let luck = rand::thread_rng().gen_range(5..15);
        Self {
            hp: 100,
            ac: 10,
            luck,
            pos,
            state: PlayerState::Walking,
            carrying: "Sword".to_string(),
            items: vec!["Health potion".to_string()],
        }
    }

    pub fn hit_by(&mut self, damage: i32) {
        let remaining_damage = damage - self.ac;
        self.ac -= damage;
        self.hp -= remaining_damage;
    }

    #[must_use]
    pub fn attack(&mut self, enemy: &mut Enemy) -> i32 {
        let damage = rand::thread_rng().gen_range(0..self.luck + 1);
        enemy.hit_by(damage);
        damage
    }

    pub fn equip(&mut self, i: usize) -> Result<(), ()> {
        let this = &self.items.clone();
        let item = this.get(i);
        if let Some(item) = &item {
            let old = self.carrying.clone();
            self.items.remove(i);
            self.carrying = item.to_string();
            self.items.push(old);
            return Ok(());
        }
        Err(())
    }

    pub fn check_sourrounding(&mut self, enemies: &Vec<Pos>) {
        let mut out = vec![];
        for &pos in enemies {
            let d = distance(self.pos, pos);
            if d < 3.0 {
                out.push(pos);
            }
        }
        if !out.is_empty() {
            self.state = PlayerState::Combat(out);
        } else {
            self.state = PlayerState::Walking;
        }
    }
}
