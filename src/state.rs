use std::collections::HashMap;

use symmetric_shadowcasting::{compute_fov, Pos};
use tatami_dungeon::{Dungeon, GenerateDungeonParams, Tile};

use crate::{distance, player::Player, Block, Enemy};

#[derive(Debug)]
pub struct State<'a> {
    pub map: HashMap<Pos, Block>,
    pub enemies: HashMap<Pos, Enemy>,
    pub player: &'a mut Player,
    pub log: Vec<String>,
}

impl<'a> State<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        Self {
            player,
            map: Default::default(),
            enemies: Default::default(),
            log: vec![],
        }
    }
    pub fn update(&mut self) {
        self.player.check_sourrounding(&self.compute_enemies());
    }
    pub fn event(&mut self, event: String) {
        self.log.push(event);
    }
    pub fn compute_walls(&self) -> Vec<Pos> {
        let mut fov: Vec<Pos> = vec![];
        let mut is_visible = |pos| fov.push(pos);
        let mut v = |pos: Pos| {
            distance(pos, self.player.pos) > 10.0 || { self.map.get(&pos) == Some(&Block::Wall) }
        };
        compute_fov(self.player.pos, &mut v, &mut is_visible);
        fov
    }

    pub fn compute_enemies(&self) -> Vec<Pos> {
        let mut fov = vec![];
        let mut is_visible = |pos: Pos| {
            if self.enemies.contains_key(&pos) {
                fov.push(pos);
            }
        };
        let mut v = |pos: Pos| {
            distance(pos, self.player.pos) > 10.0 || { self.map.get(&pos) == Some(&Block::Wall) }
        };
        compute_fov(self.player.pos, &mut v, &mut is_visible);
        fov
    }

    pub fn reset(&mut self) {
        self.player.pos = (1, 1);
        let params = GenerateDungeonParams {
            max_enemies_per_room: 1,
            squareness: 0.1,
            min_teleporters_per_floor: 10,
            max_teleporters_per_floor: 15,
            num_floors: 1,
            dimensions: (32, 32),
            ..Default::default()
        };
        let dungeon = Dungeon::generate_with_params(params);
        let floor = &dungeon.floors[0];
        let mut map = HashMap::new();
        let mut farthes = f32::MIN;
        let mut far_pos = (farthes as isize, farthes as isize);
        for (x, col) in floor.tiles.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    Tile::Floor => {
                        let dis = distance(self.player.pos, (x as isize, y as isize));
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
            r.enemies.iter().step_by(2).map(|enemy| {
                let p = (enemy.position.x as isize, enemy.position.y as isize);
                (p, Enemy::new(32, p))
            })
        }));
        let _items = HashMap::<i32, i32>::from_iter(floor.rooms.iter().flat_map(|r| {
            r.items.iter().map(|i| {
                dbg!(i);
                (0, 1)
            })
        }));
        map.insert(far_pos, Block::Exit);
        self.map = map;
        self.enemies = enemies;
        loop {
            if self.map.get(&self.player.pos) == Some(&Block::Wall) {
                self.player.pos = crate::add(&self.player.pos, &(1, 1));
            } else {
                return;
            }
        }
    }
}
