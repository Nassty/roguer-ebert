use std::collections::HashMap;

use symmetric_shadowcasting::{compute_fov, Pos};
use tatami_dungeon::{Dungeon, GenerateDungeonParams, Tile};

use crate::{distance, Block, Enemy};

#[derive(Default)]
pub struct State {
    pub map: HashMap<Pos, Block>,
    pub enemies: HashMap<Pos, Enemy>,
    pub center: Pos,
}

impl State {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn compute_walls(&self) -> Vec<Pos> {
        let mut fov: Vec<Pos> = vec![];
        let mut is_visible = |pos| fov.push(pos);
        let mut v = |pos: Pos| {
            distance(pos, self.center) > 40 || { self.map.get(&pos) == Some(&Block::Wall) }
        };
        compute_fov(self.center, &mut v, &mut is_visible);
        fov
    }

    pub fn compute_enemies(&self, map: &HashMap<Pos, Enemy>) -> Vec<Pos> {
        let mut fov = vec![];
        let mut is_visible = |pos: Pos| {
            if map.contains_key(&pos) {
                fov.push(pos);
            }
        };
        let mut v = |pos: Pos| {
            distance(pos, self.center) > 30 || { self.map.get(&pos) == Some(&Block::Wall) }
        };
        compute_fov(self.center, &mut v, &mut is_visible);
        fov
    }

    pub fn reset(&mut self) {
        self.center = (1, 1);
        let params = GenerateDungeonParams {
            squareness: 0.1,
            min_teleporters_per_floor: 10,
            max_teleporters_per_floor: 15,
            num_floors: 1,
            max_enemies_per_room: 1,
            dimensions: (32, 32),
            ..Default::default()
        };
        let dungeon = Dungeon::generate_with_params(params);
        let floor = &dungeon.floors[0];
        let mut map = HashMap::new();
        let mut farthes = isize::MIN;
        let mut far_pos = (farthes, farthes);
        for (x, col) in floor.tiles.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    Tile::Floor => {
                        let dis = distance(self.center, (x as isize, y as isize));
                        if dis > farthes {
                            farthes = dis;
                            far_pos = (x as isize, y as isize);
                        }
                    }
                    Tile::Wall => {
                        map.insert((x as isize, y as isize), Block::Wall);
                    }
                }
            }
        }
        let tps: HashMap<u32, (u32, Pos)> = HashMap::from_iter(floor.rooms.iter().flat_map(|k| {
            k.teleporters.iter().map(|t| {
                (
                    t.id,
                    (t.connected, (t.position.x as isize, t.position.y as isize)),
                )
            })
        }));

        for (target, teleporter) in tps.values() {
            map.insert(*teleporter, Block::Teleporter(tps.get(target).unwrap().1));
        }
        let enemies: HashMap<Pos, Enemy> = HashMap::from_iter(floor.rooms.iter().flat_map(|r| {
            r.enemies.iter().map(|enemy| {
                let p = (enemy.position.x as isize, enemy.position.y as isize);
                (p.clone(), Enemy::new(32, p))
            })
        }));
        map.insert(far_pos, Block::Exit);
        self.map = map;
        self.enemies = enemies;
        loop {
            if self.map.get(&self.center) == Some(&Block::Wall) {
                self.center = crate::add(&self.center, &(1, 1));
            } else {
                return;
            }
        }
    }
}
