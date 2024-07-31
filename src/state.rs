use std::collections::HashMap;

use crate::utils::Pos;
use bounded_vec_deque::BoundedVecDeque;
use symmetric_shadowcasting::{compute_fov, Pos as SPos};
use tatami_dungeon::{Dungeon, GenerateDungeonParams, Tile};

use crate::{distance, player::Player, Block, Enemy};

#[derive(Debug)]
pub enum EventType {
    DamageDealt,
    DamageTaken,
    Teleport,
}

#[derive(Debug)]
pub struct State<'a> {
    pub map: HashMap<Pos, Block>,
    pub enemies: HashMap<Pos, Enemy>,
    pub player: &'a mut Player,
    pub log: BoundedVecDeque<(String, EventType)>,
}

impl<'a> State<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        Self {
            player,
            map: Default::default(),
            enemies: Default::default(),
            log: BoundedVecDeque::new(8),
        }
    }
    pub fn update(&mut self) {
        self.player.check_sourrounding(&self.compute_enemies());
    }
    pub fn event(&mut self, event: String, etype: EventType) {
        self.log.push_front((event, etype));
    }
    pub fn compute_walls(&self) -> Vec<Pos> {
        let mut fov: Vec<Pos> = vec![];
        let mut is_visible = |pos: SPos| fov.push(pos.into());
        let mut v = |pos: SPos| {
            distance(pos.into(), self.player.pos) > 10.0 || {
                self.map.get(&pos.into()) == Some(&Block::Wall)
            }
        };
        compute_fov(self.player.pos.as_tuple(), &mut v, &mut is_visible);
        fov
    }

    pub fn compute_enemies(&self) -> Vec<Pos> {
        let mut fov = vec![];
        let mut is_visible = |pos: SPos| {
            if self.enemies.contains_key(&pos.into()) {
                fov.push(pos.into());
            }
        };
        let mut v = |pos: SPos| {
            distance(pos.into(), self.player.pos) > 10.0 || {
                self.map.get(&pos.into()) == Some(&Block::Wall)
            }
        };
        compute_fov(self.player.pos.as_tuple(), &mut v, &mut is_visible);
        fov
    }

    pub fn reset(&mut self) {
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
        self.player.pos = (
            dungeon.player_position.x as isize,
            dungeon.player_position.y as isize,
        )
            .into();
        let floor = &dungeon.floors[0];
        let mut map = HashMap::new();
        let mut farthes = f32::MIN;
        let mut far_pos = (farthes as isize, farthes as isize);
        for (x, col) in floor.tiles.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    Tile::Floor => {
                        let dis = distance(self.player.pos, (x as isize, y as isize).into());
                        if dis > farthes {
                            farthes = dis;
                            far_pos = (x as isize, y as isize);
                        }
                    }
                    Tile::Wall => {
                        map.insert((x as isize, y as isize).into(), Block::Wall);
                    }
                }
            }
        }
        let tps: HashMap<u32, (u32, Pos)> = HashMap::from_iter(floor.rooms.iter().flat_map(|k| {
            k.teleporters.iter().map(|t| {
                (
                    t.id,
                    (
                        t.connected,
                        (t.position.x as isize, t.position.y as isize).into(),
                    ),
                )
            })
        }));

        for (target, teleporter) in tps.values() {
            map.insert(*teleporter, Block::Teleporter(tps.get(target).unwrap().1));
        }
        let enemies: HashMap<Pos, Enemy> = HashMap::from_iter(floor.rooms.iter().flat_map(|r| {
            r.enemies.iter().step_by(2).map(|enemy| {
                let p = (enemy.position.x as isize, enemy.position.y as isize).into();
                (p, Enemy::new(32, p))
            })
        }));
        let _items = HashMap::<i32, i32>::from_iter(floor.rooms.iter().flat_map(|r| {
            r.items.iter().map(|i| {
                dbg!(i);
                (0, 1)
            })
        }));
        map.insert(far_pos.into(), Block::Exit);
        self.map = map;
        self.enemies = enemies;
    }
}
