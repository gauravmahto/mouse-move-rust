use rand::Rng;
use std::time::Duration;

use anyhow::Result;
use crossbeam_channel::{bounded, select, tick, Receiver};
use ctrlc;
use mouse_rs::Mouse;
// use mouse_rs::{types::keys::Keys, Mouse};

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

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() -> Result<()> {
    let wait_sec = 10;

    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(wait_sec));

    let mut last_mouse_pos: (i32, i32) = (0, 0);

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
                last_mouse_pos = move_and_press(last_mouse_pos);
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}
