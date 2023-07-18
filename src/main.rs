use std::{thread, time};

use mouse_rs::{types::keys::Keys, Mouse};

fn move_and_press() {
    let mouse = Mouse::new();
    // mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.press(&Keys::RIGHT).expect("Unable to press button");
    // mouse
    //     .release(&Keys::RIGHT)
    //     .expect("Unable to release button");
}

fn main() {
    println!("Program started!");

    let ten_secs = time::Duration::from_secs(10);

    loop {
        move_and_press();
        thread::sleep(ten_secs);
    }
}
