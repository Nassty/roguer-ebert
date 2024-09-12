use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::HashMap;

pub trait EditableEntity {
    fn heal(&mut self, value: &Value);
    fn damage(&mut self, value: &Value);
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Melee,
    Ingredient,
    HealPotion,
}
impl Distribution<ItemType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemType {
        match rng.gen_range(0..3) {
            0 => ItemType::Melee,
            1 => ItemType::Ingredient,
            2 => ItemType::HealPotion,
            _ => unreachable!(),
        }
    }
}

struct SwordName<'a>(&'a str);

impl<'a> Distribution<SwordName<'a>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SwordName<'a> {
        match rng.gen_range(0..5) {
            0 => SwordName("Wooden Sword"),
            1 => SwordName("Copper Sword"),
            2 => SwordName("Iron Sword"),
            3 => SwordName("Magic Sword"),
            4 => SwordName("Sword Of Destiny"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Action {
    Heal,
    Damage,
    None,
}
pub type Value = i32;

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    ty: ItemType,
    actions: HashMap<Action, Value>,
}

impl Distribution<Item> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Item {
        let ty: ItemType = rand::random();
        match ty {
            ItemType::Melee => {
                let name: SwordName = rand::random();
                let actions = HashMap::from([(Action::Damage, rng.gen_range(0..30))]);
                Item {
                    ty,
                    name: name.0.to_string(),
                    actions,
                }
            }
            ItemType::Ingredient => Item {
                ty,
                name: "Random Ingridient".to_string(),
                actions: HashMap::new(),
            },
            ItemType::HealPotion => {
                let name = "Heal Potion".to_string();
                let actions = HashMap::from([(Action::Damage, rng.gen_range(0..30))]);

                Item { ty, name, actions }
            }
        }
    }
}

impl Item {
    pub fn new(name: String, ty: ItemType, actions: HashMap<Action, Value>) -> Self {
        Self { name, ty, actions }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn apply(&self, user: &mut impl EditableEntity, target: &mut impl EditableEntity) {
        for (action, value) in &self.actions {
            match action {
                Action::Heal => user.heal(value),
                Action::Damage => target.damage(value),
                Action::None => {}
            }
        }
    }
}
