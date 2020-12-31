use crate::chip8::Chip8;
use sdl2::keyboard::Keycode;
pub struct Input {
    pub keycode: Keycode,
}

// 1, 2, 3, C   1 2 3 4
// 4, 5, 6, D   Q W E R  
// 7, 7, 9, E   A S D F
// A, 0, B, F   Y X C V

impl Input {
    pub fn handle_key_down(keycode: Keycode, cpu: &mut Chip8) {
        println!("Key_down: {}", keycode);
        match keycode{
            Keycode::Num1 => {
                cpu.keyboard[0x1] = true;
            }
            Keycode::Num2 => {
                cpu.keyboard[0x2] = true;
            }
            Keycode::Num3 => {
                cpu.keyboard[0x3] = true;
            }
            Keycode::Num4 => {
                cpu.keyboard[0xC] = true;
            }
            Keycode::Q => {
                cpu.keyboard[0x4] = true;
            }
            Keycode::W => {
                cpu.keyboard[0x5] = true;
            }
            Keycode::E => {
                cpu.keyboard[0x6] = true;
            }
            Keycode::R => {
                cpu.keyboard[0xD] = true;
            }
            Keycode::A=> {
                cpu.keyboard[0x7] = true;
            }
            Keycode::S => {
                cpu.keyboard[0x8] = true;
            }
            Keycode::D => {
                cpu.keyboard[0x9] = true;
            }
            Keycode::F => {
                cpu.keyboard[0xE] = true;
            }
            Keycode::Y => {
                cpu.keyboard[0xA] = true;
            }
            Keycode::X => {
                cpu.keyboard[0x0] = true;
            }
            Keycode::C => {
                cpu.keyboard[0xB] = true;
            }
            Keycode::V => {
                cpu.keyboard[0xF] = true;
            }
            _ => (),
        }
    }
    pub fn handle_key_up(keycode: Keycode, cpu: &mut Chip8) {
        println!("Key_up: {}", keycode);
        match keycode{
            Keycode::Num1 => {
                
                cpu.keyboard[0x1] = false;
            }
            Keycode::Num2 => {
                cpu.keyboard[0x2] = false;
            }
            Keycode::Num3 => {
                cpu.keyboard[0x3] = false;
            }
            Keycode::Num4 => {
                cpu.keyboard[0xC] = false;
            }
            Keycode::Q => {
                cpu.keyboard[0x4] = false;
            }
            Keycode::W => {
                cpu.keyboard[0x5] = false;
            }
            Keycode::E => {
                cpu.keyboard[0x6] = false;
            }
            Keycode::R => {
                cpu.keyboard[0xD] = false;
            }
            Keycode::A=> {
                cpu.keyboard[0x7] = false;
            }
            Keycode::S => {
                cpu.keyboard[0x8] = false;
            }
            Keycode::D => {
                cpu.keyboard[0x9] = false;
            }
            Keycode::F => {
                cpu.keyboard[0xE] = false;
            }
            Keycode::Y => {
                cpu.keyboard[0xA] = false;
            }
            Keycode::X => {
                cpu.keyboard[0x0] = false;
            }
            Keycode::C => {
                cpu.keyboard[0xB] = false;
            }
            Keycode::V => {
                cpu.keyboard[0xF] = false;
            }
            _ => (),
        }
    }
}
