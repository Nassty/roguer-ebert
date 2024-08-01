use crate::utils::Pos;
use rand::{rngs::StdRng, Rng, SeedableRng};
use raylib::{
    math::rrect,
    prelude::{Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, Rectangle},
    rgui::RaylibDrawGui,
    rstr, RaylibHandle, RaylibThread,
};

use crate::{
    player,
    state::{EventType, State},
    Block, GameComponents,
};

macro_rules! translate_pos {
    ($pos:expr, $player:expr, $midpoint:expr, $vfactor:expr) => {{
        let (x, y) = $pos;
        let x = (x * $vfactor) as i32 + $midpoint.x as i32 - $player.0 as i32 * $vfactor as i32;
        let y = (y * $vfactor) as i32 + $midpoint.y as i32 - $player.1 as i32 * $vfactor as i32;
        (x, y)
    }};
}
fn inside(pos: Rectangle, size: Rectangle) -> bool {
    pos.x >= size.x
        && pos.y >= size.y
        && pos.x < size.x + size.width
        && pos.y < size.y + size.height
        && pos.x + pos.width < size.x + size.width
        && pos.y + pos.height < size.y + size.height
}

pub fn draw_end_screen(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    state: &mut State,
    components: &GameComponents,
) {
    if let Some(KeyboardKey::KEY_R) = rl.get_key_pressed() {
        state.player.xp = 0;
        state.player.hp = 100;
        state.log.clear();
        state.reset();
    }
    // TODO: REVISAR COMO CENTRAR TEXTO
    let k = rl.measure_text("DEAD", 20);
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    d.draw_text(
        "DEAD",
        components.midpoint.x as i32 - 10,
        components.midpoint.y as i32 - k / 2,
        40,
        Color::RED,
    );
}

pub fn draw_main_screen(
    d: &mut RaylibDrawHandle,
    state: &mut State,
    enemies: &Vec<Pos>,
    components: &GameComponents,
    size: Rectangle,
) {
    d.clear_background(if !components.debug {
        Color::BLACK
    } else {
        Color::WHITE
    });
    let vis = if !components.debug {
        state.compute_walls()
    } else {
        state.map.keys().cloned().collect()
    };
    for pos in vis {
        let (x, y) = translate_pos!(
            pos.as_tuple(),
            state.player.pos,
            components.midpoint,
            components.vfactor
        );
        let dest_rect = Rectangle::new(
            x as f32,
            y as f32,
            components.vfactor as f32,
            components.vfactor as f32,
        );
        if !inside(dest_rect, size) {
            continue;
        }

        match state.map.get_mut(&pos) {
            None => {
                let dest_rect = Rectangle::new(
                    x as f32,
                    y as f32,
                    components.vfactor as f32,
                    components.vfactor as f32,
                );
                let mut r = StdRng::seed_from_u64((pos.0 * pos.1) as u64);
                let k = r.gen_range(0..3);
                d.draw_texture_pro(
                    components.tex,
                    components.floor_rects.get(k).unwrap(),
                    dest_rect,
                    components.origin,
                    components.rotation,
                    Color::WHITE,
                );
            }
            Some(Block::Wall) => {
                let dest_rect = Rectangle::new(
                    x as f32,
                    y as f32,
                    components.vfactor as f32,
                    components.vfactor as f32,
                );
                let upper_block = state.map.get(&(pos + (0, -1).into()));
                let bottom_block = state.map.get(&(pos + (0, 1).into()));
                let left_block = state.map.get(&(pos + (-1, 0).into()));
                let right_block = state.map.get(&(pos + (1, 0).into()));

                let right_upper_corner = state.map.get(&(pos + (1, -1).into()));
                let right_bottom_corner = state.map.get(&(pos + (1, 1).into()));
                let left_bottom_corner = state.map.get(&(pos + (-1, 1).into()));
                let left_upper_corner = state.map.get(&(pos + (-1, -1).into()));

                let rec = match (upper_block, right_block, bottom_block, left_block) {
                    // Horizontal and vertical texture computing based on empty adjacent tile
                    (None, Some(Block::Wall), Some(Block::Wall), Some(Block::Wall)) => {
                        components.lower_horizontal_wall_rect
                    }
                    (Some(Block::Wall), None, Some(Block::Wall), Some(Block::Wall)) => {
                        components.left_vertical_wall_rect
                    }
                    (Some(Block::Wall), Some(Block::Wall), None, Some(Block::Wall)) => {
                        components.upper_horizontal_wall_rect
                    }
                    (Some(Block::Wall), Some(Block::Wall), Some(Block::Wall), None) => {
                        components.right_vertical_wall_rect
                    }

                    // Corner textures computing based on empty adjacent tiles
                    (None, None, Some(Block::Wall), Some(Block::Wall)) => {
                        components.right_upper_corner_rect
                    }
                    (None, Some(Block::Wall), None, Some(Block::Wall)) => {
                        components.lower_horizontal_wall_rect
                    } // double
                    (None, Some(Block::Wall), Some(Block::Wall), None) => {
                        components.left_upper_corner_rect
                    }
                    (Some(Block::Wall), None, None, Some(Block::Wall)) => {
                        components.right_bottom_corner_rect
                    } // double
                    (Some(Block::Wall), None, Some(Block::Wall), None) => components.player_rect, // not sure
                    (Some(Block::Wall), Some(Block::Wall), None, None) => {
                        components.left_bottom_corner_rect
                    }

                    _ => match (
                        right_upper_corner,
                        right_bottom_corner,
                        left_bottom_corner,
                        left_upper_corner,
                    ) {
                        (None, Some(Block::Wall), Some(Block::Wall), Some(Block::Wall)) => {
                            components.inner_left_bottom_corner_rect
                        }
                        (Some(Block::Wall), None, Some(Block::Wall), Some(Block::Wall)) => {
                            components.inner_left_upper_corner_rect
                        }
                        (Some(Block::Wall), Some(Block::Wall), None, Some(Block::Wall)) => {
                            components.inner_right_upper_corner_rect
                        }
                        (Some(Block::Wall), Some(Block::Wall), Some(Block::Wall), None) => {
                            components.inner_right_bottom_corner_rect
                        }
                        _ => components.enemy_rect,
                    },
                };
                d.draw_texture_pro(
                    components.tex,
                    rec,
                    dest_rect,
                    components.origin,
                    components.rotation,
                    Color::WHITE,
                );
            }
            Some(Block::Teleporter(_p)) => {
                d.draw_rectangle(
                    x,
                    y,
                    components.vfactor as i32,
                    components.vfactor as i32,
                    Color::YELLOW,
                );
            }
            Some(Block::Exit) => {
                d.draw_rectangle(
                    x,
                    y,
                    components.vfactor as i32,
                    components.vfactor as i32,
                    Color::GREEN,
                );
            }
        }
    }
    for enemy in enemies {
        let enemy = state.enemies.get(enemy).expect("enemy to be in the list");
        let (x, y) = translate_pos!(
            enemy.pos.as_tuple(),
            state.player.pos,
            components.midpoint,
            components.vfactor
        );
        let dest_rect = Rectangle::new(
            x as f32,
            y as f32,
            components.vfactor as f32,
            components.vfactor as f32,
        );
        if !inside(dest_rect, size) {
            continue;
        }
        let dest_rect = Rectangle::new(
            x as f32,
            y as f32,
            components.vfactor as f32,
            components.vfactor as f32,
        );
        d.draw_texture_pro(
            components.tex,
            components.enemy_rect,
            dest_rect,
            components.origin,
            components.rotation,
            Color::WHITE,
        );
    }

    let dest_rect = Rectangle::new(
        components.midpoint.x,
        components.midpoint.y,
        components.vfactor as f32,
        components.vfactor as f32,
    );
    d.draw_texture_pro(
        components.tex,
        components.player_rect,
        dest_rect,
        components.origin,
        state.player.get_swing_deg(),
        Color::WHITE,
    );
}

pub fn draw_ui(d: &mut RaylibDrawHandle, state: &State, size: &Rectangle) {
    let banner = match &state.player.state {
        player::PlayerState::Walking => {
            format!(
                "
Walking (Hp: {}, XP: {})

Carrying: {}

",
                &state.player.hp, &state.player.xp, &state.player.carrying
            )
        }
        player::PlayerState::Combat(_) => {
            format!(
                "
In Combat (Hp: {0})

(p) - Attack with {1}
(o) - Use {1}

",
                &state.player.hp, &state.player.carrying
            ) + &state
                .player
                .items
                .iter()
                .enumerate()
                .map(|(i, v)| format!("({}) - Equip {}", i + 1, v))
                .collect::<Vec<String>>()
                .join("\n")
        }
    };
    for (i, line) in banner.lines().enumerate() {
        let height = (size.y as i32) + (20 * i) as i32;
        d.draw_text(line, size.x as i32, height, 20, Color::RAYWHITE);
    }
}
pub fn draw_log(d: &mut RaylibDrawHandle, state: &State, size: &Rectangle) {
    for (i, (line, event)) in state.log.iter().enumerate() {
        let height = (size.y as i32) + (20 * i) as i32;
        let color = match event {
            EventType::DamageDealt => Color::DARKRED,
            EventType::DamageTaken => Color::RED,
            EventType::Teleport | EventType::XP => Color::GREEN,
        };
        d.draw_text(line, size.x as i32, height, 20, color);
    }
}
