use std::collections::HashMap;

use raylib::{ffi::KeyboardKey, RaylibHandle};

pub fn debounce(
    rl: &RaylibHandle,
    key: KeyboardKey,
    delay: &mut u32,
    map: &mut HashMap<KeyboardKey, u32>,
) -> bool {
    if rl.is_key_down(key) {
        if let Some(&mut ref mut nd) = map.get_mut(&key) {
            if *nd < 1 {
                map.remove(&key);
                return true;
            }
            *nd -= 1;
        } else {
            map.insert(key, *delay);
        }
    }
    false
}
