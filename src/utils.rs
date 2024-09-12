use symmetric_shadowcasting::Pos as SymPos;

use crate::state::State;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Pos(pub isize, pub isize);

impl Pos {
    pub fn as_tuple(&self) -> SymPos {
        (self.0, self.1)
    }
    pub fn around(&self) -> Vec<Self> {
        (-1..=1)
            .flat_map(|y| (-1..=1).map(move |x| Self::from((self.0 + x, self.1 + y))))
            .collect()
    }
}

impl From<(isize, isize)> for Pos {
    fn from(value: (isize, isize)) -> Self {
        Pos(value.0, value.1)
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1).into()
    }
}

pub fn distance(a: Pos, b: Pos) -> f32 {
    ((a.0 - b.0).pow(2) as f32 + (a.1 - b.1).pow(2) as f32).sqrt()
}

pub fn check_collision(state: &mut State, delta: &Pos) {
    let newpos = &(state.player.pos + *delta);
    match state.map.get(newpos) {
        Some(Block::Wall) => {}
        _ => {
            state.player.cicle_swing();
            state.player.pos = *newpos;
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Block {
    Wall,
    Exit,
    Teleporter(Pos),
}
