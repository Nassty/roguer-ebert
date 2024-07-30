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

#[macro_export]
macro_rules! debounce_key_action {
    ($key:expr => $rl:expr => $k:expr => $debounce_map:expr => $state:expr => $next:expr) => {
        if $crate::keyboard::debounce(&$rl, $key, &mut $k, $debounce_map) {
            $next($state)
        }
    };
}

#[macro_export]
macro_rules! debounce_key_move {
    ($key:expr => $delta:expr => $rl:expr => $k:expr => $debounce_map:expr => $state:expr => $flag:expr) => {
        if $crate::keyboard::debounce(&$rl, $key, &mut $k, $debounce_map) {
            $flag = true;
            check_collision(&mut $state, &$delta)
        }
    };
}
