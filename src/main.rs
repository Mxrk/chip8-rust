mod chip8;
mod display;
mod input;

use input::Input;

use chip8::Chip8;
use display::Display;
use sdl2::event::Event;
use std::fs::File;
use std::io::Read;
use std::{thread, time};
fn main() {
    // let input = input::Input();
    let mut chip = Chip8::init();
    let mut file = File::open("test.chip8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");
    // println!("{:?}", data);

    chip.load_game(&data);

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);
    let mut events = sdl_context.event_pump().unwrap();
    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(k) => Input::handle_key_down(k, &mut chip),
                    None => continue,
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(k) => Input::handle_key_up(k, &mut chip),
                    None => continue,
                },
                _ => continue,
            }
        }
        chip.cycle();
        if chip.update_screen {
            chip.update_screen = false;
            display.draw(&chip.vram);
        }
        // println!("Tick")
        let ten_millis = time::Duration::from_millis(20);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
    }
}
