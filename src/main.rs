use std::collections::HashMap;

use raylib::prelude::*;
use symmetric_shadowcasting::{compute_fov, Pos};

use irrgarten::Maze;

enum Block {
    Wall,
    Exit,
}

fn build_map(user_pos: Pos, size: (usize, usize)) -> HashMap<Pos, Block> {
    let mut map = HashMap::new();
    let mut rng = rand::thread_rng();
    let maze = Maze::new(size.0, size.1).unwrap().generate(&mut rng);
    let mut farthes = isize::MIN;
    let mut far_pos = (farthes, farthes);
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze[x][y] > 0 {
                map.insert((x as isize, y as isize), Block::Wall);
            } else {
                let dis = distance(user_pos, (x as isize, y as isize));
                if dis > farthes {
                    farthes = dis;
                    far_pos = (x as isize, y as isize);
                }
            }
        }
    }
    println!("{:?}", far_pos);
    map.insert(far_pos, Block::Exit);
    map
}

pub fn distance(a: Pos, b: Pos) -> isize {
    (a.1 - b.1).abs() + (a.0 - b.0).abs()
}

fn compute(map: &HashMap<Pos, Block>, center: Pos) -> Vec<Pos> {
    let mut fov: Vec<Pos> = vec![];
    let mut is_visible = |pos| fov.push(pos);
    let mut v = |pos: Pos| distance(pos, center) > 20 || map.contains_key(&pos);
    compute_fov(center, &mut v, &mut is_visible);
    fov
}

fn add(k: &Pos, y: &Pos) -> Pos {
    (k.0 + y.0, k.1 + y.1)
}

fn check_collision(map: &HashMap<Pos, Block>, center: &mut Pos, delta: &Pos) {
    let newpos = add(center, delta);
    let block = map.get(&newpos);
    if let Some(Block::Wall) = block {
    } else {
        *center = newpos;
    }
}
fn main() {
    let width = 1680;
    let height = 1050;
    let midpoint = (width / 2, height / 2);
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Hello, World")
        .build();
    let mut center = (1, 1);
    let mut maze_size = (5, 5);
    let mut map = build_map(center, maze_size);
    let mut debug = false;
    let mut vfactor = 10;

    while !rl.window_should_close() {
        let pressed_key = rl.get_key_pressed();
        if let Some(k) = pressed_key {
            match k {
                KeyboardKey::KEY_A => {
                    let delta = (-1, 0);
                    check_collision(&map, &mut center, &delta)
                }
                KeyboardKey::KEY_W => {
                    let delta = (0, -1);
                    check_collision(&map, &mut center, &delta)
                }
                KeyboardKey::KEY_D => {
                    let delta = (1, 0);
                    check_collision(&map, &mut center, &delta)
                }
                KeyboardKey::KEY_S => {
                    let delta = (0, 1);
                    check_collision(&map, &mut center, &delta)
                }
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
            compute(&map, center)
        } else {
            map.keys().cloned().collect()
        };
        if let Some(&Block::Exit) = map.get(&center) {
            maze_size = (maze_size.0 + 2, maze_size.1 + 2);
            center = (1, 1);
            map = build_map(center, maze_size);
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(if !debug { Color::BLACK } else { Color::WHITE });
        for pos in vis {
            let (x, y) = pos;
            let x = (x * vfactor) as i32 + midpoint.0 - center.0 as i32 * vfactor as i32;
            let y = (y * vfactor) as i32 + midpoint.1 - center.1 as i32 * vfactor as i32;
            match map.get(&pos) {
                Some(Block::Wall) => {
                    d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::BLUE);
                }
                Some(Block::Exit) => {
                    d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::GREEN);
                }
                None => {
                    d.draw_rectangle(x, y, vfactor as i32, vfactor as i32, Color::WHITE);
                }
            }
        }
        d.draw_rectangle(
            midpoint.0,
            midpoint.1,
            vfactor as i32,
            vfactor as i32,
            Color::BLACK,
        );
        d.draw_text(format!("{center:?}").as_str(), 0, 0, 20, Color::BLACK);
    }
}
