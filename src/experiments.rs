// todo: check out "ktrl" for how it uses evdev
// todo: also consider using libinput
// todo: what is "udev"?
// >>> draw a small picture of how all this crap fits together
//      - decide which of these libs is actually what I want/need 

// use enigo::{Enigo, MouseControllable, MouseButton};

// fn _enigo_mouse_input_hello_world() {
//     let mut enigo = Enigo::new();
//     enigo.mouse_move_to(0, 0);
//     enigo.mouse_click(MouseButton::Left);
// }

// fn _inputbot_hw() {
//     Numrow1Key
// }

// fn main() {
//     _inputbot_hw()
// }

use inputbot::{KeySequence, KeybdKey::*, MouseButton::*, MouseCursor};
use std::{thread::sleep, time::Duration};

fn main() {
    // Bind the number 1 key your keyboard to a function that types 
    // "Hello, world!" when pressed.
    // Numrow1Key.bind(|| KeySequence("Hello, world!").send());
    BackspaceKey.bind(|| KeySequence("Hello, world!").send());

    // KeySequence("Hello, world!").send();

    // for x in 0..=600 {
    //     MouseCursor::move_abs(x as i32, 300);
    //     sleep(Duration::from_millis(1));
    // }

    // // Bind your caps lock key to a function that starts an autoclicker.
    // CapsLockKey.bind(move || {
    //     while CapsLockKey.is_toggled() {
    //         LeftButton.press();
    //         LeftButton.release();

    //         sleep(Duration::from_millis(30));
    //     }
    // });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}

// IMPORTANT NOTE: you have to run the program as `root` for the input-grabbing to work.
// o/w it will fail silently :(
