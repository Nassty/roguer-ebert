use std::collections::HashMap;

use crate::item::{Action, EditableEntity, Item, Value};
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
    pub max_hp: i32,
    ac: i32,
    luck: i32,
    pub pos: Pos,
    pub state: PlayerState,
    pub items: Vec<Item>,
    pub carrying: Item,
    pub xp: i32,
    swing: u8,
}

impl EditableEntity for Player {
    fn heal(&mut self, value: &Value) {
        self.hp += value;
    }

    fn damage(&mut self, value: &Value) {
        self.hp -= value;
    }
}

impl Player {
    pub fn new(pos: Pos) -> Self {
        let luck = rand::thread_rng().gen_range(5..15);
        Self {
            hp: 100,
            max_hp: 100,
            ac: 10,
            xp: 0,
            luck,
            pos,
            state: PlayerState::Walking,
            carrying: Item::new(
                "Sword".into(),
                crate::item::ItemType::Melee,
                HashMap::from([(Action::Damage, 30 as Value)]),
            ),
            items: vec![],
            swing: 0,
        }
    }

    pub fn cicle_swing(&mut self) {
        self.swing = self.swing.wrapping_add(1) % 4;
    }
    pub fn get_swing_deg(&self) -> f32 {
        let angle = (self.swing as f32) / 4.0 * std::f32::consts::PI * 2.0;
        6.0 * angle.sin()
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
            self.carrying = (*item).clone();
            self.items.push(old);
            return Ok(());
        }
        Err(())
    }

    pub fn check_sourroundings(&mut self, enemies: &Vec<Pos>) {
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
