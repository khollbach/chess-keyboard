use std::{thread::sleep, time::Duration};

use inputbot::{KeybdKey::*, MouseCursor};

fn main() {
    JKey.bind(|| {
        for x in 0..=600 {
            MouseCursor::move_abs(x as i32, 300);
            sleep(Duration::from_millis(1));
        }
    });

    KKey.bind(|| {
        MouseCursor::move_rel(100, 100);
        sleep(Duration::from_millis(1));
    });

    inputbot::handle_input_events();
}
