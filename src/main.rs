use std::collections::HashMap;

use raylib::prelude::*;
mod enemy;
mod keyboard;
mod sprite_sheet;
use enemy::Enemy;

mod state;
use state::{EventType, State};
mod item;
mod player;
use player::Player;

mod draw;
use draw::{draw_end_screen, draw_log, draw_main_screen, draw_ui};

mod components;
use components::GameComponents;
mod utils;
use utils::{check_collision, distance, Block};

fn main() {
    let width = 1024;
    let height = 768;
    let debounce_map: &mut HashMap<KeyboardKey, u32> = &mut HashMap::new();
    let mut player = Player::new((1, 1).into());
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
                draw_end_screen(&mut rl, &thread, &mut state, &components);
                continue;
            }
        }
        match &state.player.state {
            player::PlayerState::Walking => {
                let mut k = 1;
                debounce_key_move!(KeyboardKey::KEY_A => (-1, 0).into() => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_W => (0, -1).into() => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_D => (1, 0).into() => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_S => (0, 1).into() => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_UP => (0, -1).into() => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_DOWN => (0, 1).into() => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_LEFT => (-1, 0).into() => rl => k => debounce_map => state => components.active_turn);
                debounce_key_move!(KeyboardKey::KEY_RIGHT => (1, 0).into() => rl => k => debounce_map => state => components.active_turn);
            }
            player::PlayerState::Combat(e) => {
                let pressed_key = rl.get_key_pressed();
                if let Some(k) = pressed_key {
                    match k {
                        KeyboardKey::KEY_P => {
                            let p = e.first().unwrap();
                            if let Some(enemy) = state.enemies.get_mut(p) {
                                let old_hp = enemy.hp;
                                let item = state.player.carrying.clone();
                                item.apply(state.player, enemy);
                                let damage = old_hp - enemy.hp;
                                state.event(
                                    format!("You attacked an enemy for {} damage", damage),
                                    EventType::DamageDealt,
                                );
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
                        _ => {}
                    }
                }
            }
        }

        let pressed_key = rl.get_key_pressed();
        if let Some(k) = pressed_key {
            match k {
                KeyboardKey::KEY_I => {
                    for pos in state.player.pos.around() {
                        if let Some(item) = state.items.get(&pos) {
                            dbg!(item);
                            break;
                        }
                    }
                }
                KeyboardKey::KEY_SPACE => {
                    components.debug = !components.debug;
                }
                KeyboardKey::KEY_MINUS => {
                    components.vfactor -= 1;
                }
                KeyboardKey::KEY_EQUAL => {
                    components.vfactor += 1;
                }
                KeyboardKey::KEY_Q => {
                    components.vfactor = 10;
                }
                KeyboardKey::KEY_O if components.debug => {
                    components.active_turn = true;
                }
                KeyboardKey::KEY_F => {
                    rl.toggle_fullscreen();
                }
                _ => {}
            }
        }

        match state
            .map
            .get(&state.player.pos)
            .or(state.teleporters_map.get(&state.player.pos))
        {
            Some(&Block::Wall) => {}
            Some(&Block::Teleporter(p)) => {
                state.event("Teleporter activated".to_string(), EventType::Teleport);
                state.player.pos = p + (-1, -1).into();
            }
            Some(&Block::Exit) => {
                state.reset();
            }

            None => {}
        }
        let mut d = rl.begin_drawing(&thread);

        let pps = state.compute_enemies();
        let k_enemies = state.enemies.clone();
        let mut new_enemies = vec![];
        let mut logs = vec![];
        for pos in &pps {
            let enemy = state.enemies.get_mut(pos);
            if enemy.is_none() {
                continue;
            }
            let enemy = enemy.unwrap();
            let damage = enemy.update(state.player, &state.map, &k_enemies, components.active_turn);
            if let Some(damage) = damage {
                logs.push(format!("Ghost hits you for {} damage", damage));
            }
            new_enemies.push(enemy.clone());
            state.enemies.remove(pos);
        }
        for log in logs {
            state.event(log, EventType::DamageTaken);
        }
        for enemy in new_enemies {
            if enemy.hp > 0 {
                state.enemies.insert(enemy.pos, enemy);
            } else {
                state.event("Enemy died".to_string(), EventType::XP);
                state.player.xp += enemy.dificulty as i32;
            }
        }
        let enemies = state.compute_enemies();
        let items = state.compute_items();
        draw_main_screen(
            &mut d,
            &mut state,
            &enemies,
            &items,
            &components,
            Rectangle::new(0.0, 0.0, width as f32, ((height / 3) * 2) as f32),
        );
        draw_ui(
            &mut d,
            &state,
            &Rectangle::new(
                0.0,
                (height / 3 * 2) as f32,
                (width / 2) as f32,
                height as f32,
            ),
        );
        draw_log(
            &mut d,
            &state,
            &Rectangle::new(
                (width / 2) as f32,
                (height / 3 * 2) as f32,
                (width / 2) as f32,
                height as f32,
            ),
        );
        components.active_turn = false;

        state.update();
    }
}
