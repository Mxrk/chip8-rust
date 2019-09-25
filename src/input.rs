use crate::chip8::Chip8;
use sdl2::keyboard::Keycode;
pub struct Input {
    pub keycode: Keycode,
}

// 1, 2, 3, C,
// 4, 5, 6, D
// 7, 7, 9, E
// A, 0, B, F

impl Input {
    pub fn handle_key_down(keycode: Keycode, cpu: &mut Chip8) {
        println!("Key_down: {}", keycode);
        cpu.keyboard[0x6] = true;
    }
    pub fn handle_key_up(keycode: Keycode, cpu: &mut Chip8) {
        println!("Key_up: {}", keycode);
        //cpu.keyboard[0x6] = false;
    }
}
