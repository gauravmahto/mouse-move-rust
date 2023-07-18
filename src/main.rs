use rand::Rng;
use std::{thread, time};

// use mouse_rs::{types::keys::Keys, Mouse};
use mouse_rs::Mouse;

fn move_and_press(last_mouse_pos: (i32, i32)) -> (i32, i32) {
    let mouse = Mouse::new();
    let mut mouse_pos = mouse.get_position().unwrap();

    if (mouse_pos.x != last_mouse_pos.0) && (mouse_pos.y != last_mouse_pos.1) {
        return (mouse_pos.x, mouse_pos.y);
    }

    let mut rng = rand::thread_rng();

    let n1 = rng.gen_range(0..1024);
    let n2 = rng.gen_range(0..1024);

    mouse.move_to(n1, n2).expect("Unable to move mouse");
    // mouse.press(&Keys::RIGHT).expect("Unable to press button");
    // mouse
    //     .release(&Keys::RIGHT)
    //     .expect("Unable to release button");

    mouse_pos = mouse.get_position().unwrap();

    return (mouse_pos.x, mouse_pos.y);
}

fn main() {
    println!("Program started!");

    let ten_secs = time::Duration::from_secs(10);

    let mut last_mouse_pos: (i32, i32) = (0, 0);

    loop {
        last_mouse_pos = move_and_press(last_mouse_pos);
        thread::sleep(ten_secs);
    }
}
