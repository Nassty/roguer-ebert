use std::collections::HashMap;

use raylib::prelude::*;
use symmetric_shadowcasting::Pos;
mod keyboard;
use keyboard::debounce;
mod enemy;
use enemy::Enemy;

mod state;
use state::State;

const SHADER: &str = include_str!("../shader.fs");

#[derive(PartialEq)]
enum Block {
    Wall,
    Exit,
    Teleporter(Pos),
}

pub fn distance(a: Pos, b: Pos) -> isize {
    (a.1 - b.1).abs() + (a.0 - b.0).abs()
}

fn add(k: &Pos, y: &Pos) -> Pos {
    (k.0 + y.0, k.1 + y.1)
}

fn check_collision(state: &mut State, delta: &Pos) {
    let newpos = add(&state.center, delta);
    match state.map.get(&newpos) {
        Some(Block::Wall) => {}
        _ => {
            state.center = newpos;
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let midpoint = (width / 2, height / 2);
    let debounce_map: &mut HashMap<KeyboardKey, u32> = &mut HashMap::new();
    let mut state = State::new();
    state.center = (1, 1);
    state.reset();
    let mut debug = false;
    let mut vfactor = 10;

    let mut flag = false;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);
    let mut shader = rl.load_shader_from_memory(&thread, None, Some(SHADER));

    let resolution_loc = shader.get_shader_location("resolution");

    let resolution: [f32; 2] = [height as f32, width as f32];
    shader.set_shader_value(resolution_loc, resolution);
    while !rl.window_should_close() {
        let mut k = 1;
        if debounce(&rl, KeyboardKey::KEY_A, &mut k, debounce_map)
            || debounce(&rl, KeyboardKey::KEY_LEFT, &mut k, debounce_map)
        {
            flag = true;
            let delta = (-1, 0);
            check_collision(&mut state, &delta)
        }
        if debounce(&rl, KeyboardKey::KEY_W, &mut k, debounce_map)
            || debounce(&rl, KeyboardKey::KEY_UP, &mut k, debounce_map)
        {
            flag = true;
            let delta = (0, -1);
            check_collision(&mut state, &delta)
        }

        if debounce(&rl, KeyboardKey::KEY_D, &mut k, debounce_map)
            || debounce(&rl, KeyboardKey::KEY_RIGHT, &mut k, debounce_map)
        {
            flag = true;
            let delta = (1, 0);
            check_collision(&mut state, &delta)
        }
        if debounce(&rl, KeyboardKey::KEY_S, &mut k, debounce_map)
            || debounce(&rl, KeyboardKey::KEY_DOWN, &mut k, debounce_map)
        {
            flag = true;
            let delta = (0, 1);
            check_collision(&mut state, &delta)
        }

        let pressed_key = rl.get_key_pressed();
        if let Some(k) = pressed_key {
            match k {
                KeyboardKey::KEY_SPACE => {
                    debug = !debug;
                }
                KeyboardKey::KEY_MINUS if debug => {
                    vfactor -= 1;
                }
                KeyboardKey::KEY_EQUAL if debug => {
                    vfactor += 1;
                }
                KeyboardKey::KEY_Q if debug => {
                    vfactor = 10;
                }
                _ => {}
            }
        }

        let vis = if !debug {
            state.compute_walls()
        } else {
            state.map.keys().cloned().collect()
        };
        match state.map.get(&state.center) {
            Some(&Block::Wall) => {}
            Some(&Block::Teleporter(p)) => {
                state.map.remove(&state.center);
                state.center = add(&p, &(-1, -1));
            }
            Some(&Block::Exit) => {
                state.reset();
            }

            None => {}
        }
        let mut d = rl.begin_drawing(&thread);
        {
            let mut d = d.begin_shader_mode(&shader);
            d.clear_background(if !debug { Color::BLACK } else { Color::WHITE });

            for pos in vis {
                let (x, y) = pos;
                let x = (x * vfactor) as i32 + midpoint.0 - state.center.0 as i32 * vfactor as i32;
                let y = (y * vfactor) as i32 + midpoint.1 - state.center.1 as i32 * vfactor as i32;
                match state.map.get_mut(&pos) {
                    Some(Block::Wall) => {
                        d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::BLUE);
                    }
                    Some(Block::Teleporter(_p)) => {
                        d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::YELLOW);
                    }
                    Some(Block::Exit) => {
                        d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::GREEN);
                    }

                    None => {
                        d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::WHITE);
                    }
                }
            }
        }
        let k_enemies = state.enemies.clone();
        let pps = state.compute_enemies(&state.enemies);
        for pos in &pps {
            let enemy = state.enemies.get_mut(pos).unwrap();
            let (x, y) = enemy.pos;
            let x = (x * vfactor) as i32 + midpoint.0 - state.center.0 as i32 * vfactor as i32;
            let y = (y * vfactor) as i32 + midpoint.1 - state.center.1 as i32 * vfactor as i32;
            d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::RED);
            enemy.update(state.center, &state.map, &k_enemies, flag);
        }
        flag = false;
        d.draw_rectangle(
            midpoint.0,
            midpoint.1,
            vfactor as i32,
            vfactor as i32,
            Color::BLACK,
        );
        d.draw_fps(0, 0)
    }
}
