use std::collections::HashMap;

trait EditableEntity {
    fn heal(&mut self, value: &Value);
    fn damage(&mut self, value: &Value);
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Melee,
    Range,
    Ingredient,
    HealPotion,
    EnhancePotion,
}
#[derive(Debug, Clone)]
pub enum Action {
    Heal,
    Damage,
    None,
}
type Value = i32;

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    ty: ItemType,
    actions: HashMap<Action, Value>,
}

impl Item {
    pub fn new(name: String, ty: ItemType, actions: HashMap<Action, Value>) -> Self {
        Self { name, ty, actions }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn apply(&self, target: &mut impl EditableEntity) {
        for (action, value) in &self.actions {
            match action {
                Action::Heal => target.heal(value),
                Action::Damage => target.damage(value),
                Action::None => {}
            }
        }
    }
}
