use std::collections::HashMap;

use rand::{rngs::StdRng, Rng, SeedableRng};
use raylib::prelude::*;
use symmetric_shadowcasting::Pos;
mod enemy;
mod keyboard;
mod sprite_sheet;
use enemy::Enemy;
use sprite_sheet::SpriteSheet;

mod state;
use state::State;
mod player;
use player::Player;

#[derive(PartialEq, Debug)]
enum Block {
    Wall,
    Exit,
    Teleporter(Pos),
}

macro_rules! translate_pos {
    ($pos:expr, $player:expr, $midpoint:expr, $vfactor:expr) => {{
        let (x, y) = $pos;
        let x = (x * $vfactor) as i32 + $midpoint.x as i32 - $player.0 as i32 * $vfactor as i32;
        let y = (y * $vfactor) as i32 + $midpoint.y as i32 - $player.1 as i32 * $vfactor as i32;
        (x, y)
    }};
}

pub fn distance(a: Pos, b: Pos) -> f32 {
    ((a.0 - b.0).pow(2) as f32 + (a.1 - b.1).pow(2) as f32).sqrt()
}

fn add(k: &Pos, y: &Pos) -> Pos {
    (k.0 + y.0, k.1 + y.1)
}

fn check_collision(state: &mut State, delta: &Pos) {
    let newpos = add(&state.player.pos, delta);
    match state.map.get(&newpos) {
        Some(Block::Wall) => {}
        _ => {
            state.player.pos = newpos;
        }
    }
}

fn inside(pos: Rectangle, size: Rectangle) -> bool {
    pos.x >= size.x
        && pos.y >= size.y
        && pos.x < size.x + size.width
        && pos.y < size.y + size.height
        && pos.x + pos.width < size.x + size.width
        && pos.y + pos.height < size.y + size.height
}

fn draw_main_screen(
    d: &mut RaylibDrawHandle,
    state: &mut State,
    enemies: &Vec<Pos>,
    components: &GameComponents,
    size: Rectangle,
) {
    let vis = if !components.debug {
        state.compute_walls()
    } else {
        state.map.keys().cloned().collect()
    };
    for pos in vis {
        let (x, y) = translate_pos!(
            pos,
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
                let upper_block = state.map.get(&add(&pos, &(0, -1)));
                let bottom_block = state.map.get(&add(&pos, &(0, 1)));
                let left_block = state.map.get(&add(&pos, &(-1, 0)));
                let right_block = state.map.get(&add(&pos, &(1, 0)));

                let right_upper_corner = state.map.get(&add(&pos, &(1, -1)));
                let right_bottom_corner = state.map.get(&add(&pos, &(1, 1)));
                let left_bottom_corner = state.map.get(&add(&pos, &(-1, 1)));
                let left_upper_corner = state.map.get(&add(&pos, &(-1, -1)));

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
            enemy.pos,
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
        components.rotation,
        Color::WHITE,
    );
}

#[allow(unused)]
fn draw_ui(d: &mut RaylibDrawHandle, state: &State, size: &Rectangle) {
    let banner = match &state.player.state {
        player::PlayerState::Walking => {
            format!(
                "
Walking (Hp: {0})

Carrying: {1}
",
                &state.player.hp, &state.player.carrying
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
        d.draw_text(line, 0, height, 20, Color::RED);
    }
}

#[allow(dead_code)]
struct GameComponents<'a> {
    tex: &'a Texture2D,
    lower_horizontal_wall_rect: Rectangle,
    upper_horizontal_wall_rect: Rectangle,
    left_vertical_wall_rect: Rectangle,
    right_vertical_wall_rect: Rectangle,
    left_upper_corner_rect: Rectangle,
    right_upper_corner_rect: Rectangle,
    left_bottom_corner_rect: Rectangle,
    right_bottom_corner_rect: Rectangle,

    inner_left_upper_corner_rect: Rectangle,
    inner_right_upper_corner_rect: Rectangle,
    inner_left_bottom_corner_rect: Rectangle,
    inner_right_bottom_corner_rect: Rectangle,

    enemy_rect: Rectangle,
    floor_rects: Vec<Rectangle>,
    player_rect: Rectangle,
    origin: Vector2,
    rotation: f32,
    vfactor: isize,
    debug: bool,
    active_turn: bool,
    screen_size: Vector2,
    midpoint: Vector2,
}

impl<'a> GameComponents<'a> {
    fn new(tex: &'a Texture2D, screen_size: Vector2) -> Self {
        let sheet = SpriteSheet::new(
            Vector2::new(tex.width() as f32, tex.height() as f32),
            Vector2::new(16.0, 16.0),
            1,
        );
        let lower_horizontal_wall_rect = sheet.index_to_rect(24);
        let upper_horizontal_wall_rect = sheet.index_to_rect(2);
        let left_vertical_wall_rect = sheet.index_to_rect(12);
        let right_vertical_wall_rect = sheet.index_to_rect(14);

        let left_upper_corner_rect = sheet.index_to_rect(4);
        let right_upper_corner_rect = sheet.index_to_rect(5);
        let left_bottom_corner_rect = sheet.index_to_rect(15);
        let right_bottom_corner_rect = sheet.index_to_rect(16);

        let inner_left_upper_corner_rect = sheet.index_to_rect(1);
        let inner_right_upper_corner_rect = sheet.index_to_rect(3);
        let inner_left_bottom_corner_rect = sheet.index_to_rect(23);
        let inner_right_bottom_corner_rect = sheet.index_to_rect(25);

        let enemy_rect = sheet.index_to_rect(111);
        let floor_rects = vec![
            sheet.index_to_rect(0),
            sheet.index_to_rect(11),
            sheet.index_to_rect(22),
        ];
        let player_rect = sheet.index_to_rect(88);
        let origin = Vector2::new(0.0, 0.0);
        let rotation = 0.0;
        let vfactor = 32;
        let debug = false;
        let active_turn = false;
        let midpoint = Vector2::new(screen_size.x / 2.0, screen_size.y / 2.0);
        Self {
            tex,

            lower_horizontal_wall_rect,
            upper_horizontal_wall_rect,
            left_vertical_wall_rect,
            right_vertical_wall_rect,

            left_upper_corner_rect,
            right_upper_corner_rect,
            left_bottom_corner_rect,
            right_bottom_corner_rect,

            inner_left_upper_corner_rect,
            inner_right_upper_corner_rect,
            inner_left_bottom_corner_rect,
            inner_right_bottom_corner_rect,

            enemy_rect,
            floor_rects,
            player_rect,
            origin,
            rotation,
            vfactor,
            debug,
            active_turn,
            screen_size,
            midpoint,
        }
    }
}

fn main() {
    let width = 1024;
    let height = 768;
    let debounce_map: &mut HashMap<KeyboardKey, u32> = &mut HashMap::new();
    let mut player = Player::new((1, 1));
    let mut state = State::new(&mut player);
    state.reset();

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);
    let tex = rl.load_texture(&thread, "tilemap.png").expect("texture");
    let mut components =
        GameComponents::new(&tex, Vector2::new(width as f32, (height / 3 * 2) as f32));

    while !rl.window_should_close() {
        if state.player.hp <= 0 {
            {
                if let Some(KeyboardKey::KEY_R) = rl.get_key_pressed() {
                    state.reset();
                    state.player.hp = 100;
                }
                // TODO: REVISAR COMO CENTRAR TEXTO
                let k = rl.measure_text("DEAD", 20);
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text(
                    "DEAD",
                    components.midpoint.x as i32 - 10,
                    components.midpoint.y as i32 - k / 2,
                    40,
                    Color::RED,
                );
                continue;
            }
        }
        match &state.player.state {
            player::PlayerState::Walking => {
                let mut k = 1;
                debounce_key_move!(KeyboardKey::KEY_A => (-1, 0) => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_W => (0, -1) => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_D=> (1, 0) => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_S => (0, 1) => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_UP => (0, -1) => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_DOWN => (0, 1) => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_LEFT => (-1, 0) => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_RIGHT => (1, 0) => rl => k => debounce_map => state => components.active_turn);
            }
            player::PlayerState::Combat(e) => {
                let pressed_key = rl.get_key_pressed();
                if let Some(k) = pressed_key {
                    match k {
                        KeyboardKey::KEY_P => {
                            let p = e.first().unwrap();
                            if let Some(enemy) = state.enemies.get_mut(p) {
                                state.player.attack(enemy);
                                components.active_turn = true;
                            } else {
                                println!("Enemy not found");
                            }
                        }
                        KeyboardKey::KEY_ONE => {
                            components.active_turn = true;
                            let _ = state.player.equip(0);
                        }
                        KeyboardKey::KEY_TWO => {
                            components.active_turn = true;
                            let _ = state.player.equip(1);
                        }
                        KeyboardKey::KEY_THREE => {
                            components.active_turn = true;
                            let _ = state.player.equip(2);
                        }
                        KeyboardKey::KEY_FOUR => {
                            components.active_turn = true;
                            let _ = state.player.equip(3);
                        }
                        KeyboardKey::KEY_F => {
                            rl.toggle_fullscreen();
                        }
                        _ => {}
                    }
                }
            }
        }

        let pressed_key = rl.get_key_pressed();
        if let Some(k) = pressed_key {
            match k {
                KeyboardKey::KEY_SPACE => {
                    components.debug = !components.debug;
                }
                KeyboardKey::KEY_MINUS if components.debug => {
                    components.vfactor -= 1;
                }
                KeyboardKey::KEY_EQUAL if components.debug => {
                    components.vfactor += 1;
                }
                KeyboardKey::KEY_Q if components.debug => {
                    components.vfactor = 10;
                }
                KeyboardKey::KEY_O if components.debug => {
                    components.active_turn = true;
                }
                _ => {}
            }
        }

        match state.map.get(&state.player.pos) {
            Some(&Block::Wall) => {}
            Some(&Block::Teleporter(p)) => {
                state.map.remove(&state.player.pos);
                state.player.pos = add(&p, &(-1, -1));
            }
            Some(&Block::Exit) => {
                state.reset();
            }

            None => {}
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(if !components.debug {
            Color::BLACK
        } else {
            Color::WHITE
        });

        let pps = state.compute_enemies();
        let k_enemies = state.enemies.clone();
        let mut new_enemies = vec![];
        for pos in &pps {
            let enemy = state.enemies.get_mut(pos);
            if enemy.is_none() {
                continue;
            }
            let enemy = enemy.unwrap();
            enemy.update(state.player, &state.map, &k_enemies, components.active_turn);
            new_enemies.push(enemy.clone());
            state.enemies.remove(pos);
        }
        for enemy in new_enemies {
            if enemy.hp > 0 {
                state.enemies.insert(enemy.pos, enemy);
            }
        }
        let enemies = state.compute_enemies();
        draw_main_screen(
            &mut d,
            &mut state,
            &enemies,
            &components,
            Rectangle::new(0.0, 0.0, width as f32, ((height / 3) * 2) as f32),
        );
        draw_ui(
            &mut d,
            &state,
            &Rectangle::new(0.0, (height / 3 * 2) as f32, width as f32, height as f32),
        );
        components.active_turn = false;

        state.update();
    }
}
