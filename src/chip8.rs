use rand::Rng;

const START: usize = 0x200;
const VRAMH: usize = 32;
const VRAMW: usize = 64;

const FONT: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
  0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
  0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
  0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
  0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Chip8 {
  pub memory: [u8; 4096],
  pub pc: usize,
  pub i: usize,
  pub v: [u8; 16],
  pub stack: [u16; 16],
  pub sp: usize,
  pub vram: [[u8; VRAMW]; VRAMH],
  pub keyboard: [bool; 16],
  pub delay: u8,
  pub sound: u8,
  pub update_screen: bool,
}

impl Chip8 {
  pub fn init() -> Self {
    let mut mem = [0; 4096];
    for i in 0..FONT.len() {
      mem[i] = FONT[i];
    }
    Chip8 {
      pc: 0x200,
      i: 0,
      v: [0; 16],
      memory: mem,
      stack: [0; 16],
      sp: 0,
      vram: [[0; VRAMW]; VRAMH],
      keyboard: [false; 16],
      delay: 0,
      sound: 0,
      update_screen: true,
    }
  }
  pub fn load_game(&mut self, game: &Vec<u8>) {
    //write into 512 (0x200)+
    self.memory[START..START + game.len()].copy_from_slice(game);
  }
  pub fn cycle(&mut self) {
    println!("Delay: {}", self.delay);
    if self.delay > 0 {
      self.delay -= 1
    }
    if self.sound > 0 {
      self.sound -= 1
    }
    self.opcode()
  }

  pub fn next_instruction(&mut self) {
    self.pc += 2;
  }
  pub fn opcode(&mut self) {
    let opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;
    //op1:6 op2:A op3:0 op4:0
    //Set the unwanted parts to zero with and operation
    let nibbles = (
      (opcode & 0xF000) >> 12 as u8,
      (opcode & 0x0F00) >> 8 as u8,
      (opcode & 0x00F0) >> 4 as u8,
      (opcode & 0x000F) as u8,
    );
    let a = nibbles.1 as usize;
    let b = nibbles.2 as usize;
    let c = nibbles.3 as usize;
    let nnn = (opcode & 0x0FFF) as usize;
    println!("0x{:X?}", opcode);
    let kk = (opcode & 0x00FF) as u8;
    //println!("Stack: {:?}", self.stack);
    match nibbles {
      (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
      (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
      (0x00, _, _, _) => self.op_0nnn(nnn),
      (0x01, _, _, _) => self.op_1nnn(nnn),
      (0x02, _, _, _) => self.op_2nnn(nnn),
      (0x03, _, _, _) => self.op_3xkk(a, kk),
      (0x04, _, _, _) => self.op_4xkk(a, kk),
      (0x05, _, _, 0x00) => self.op_5xy0(a, b),
      (0x06, _, _, _) => self.op_6xnn(a, kk),
      (0x07, _, _, _) => self.op_7xkk(a, kk),
      (0x08, _, _, 0x00) => self.op_8xy0(a, b),
      (0x08, _, _, 0x01) => self.op_8xy1(a, b),
      (0x08, _, _, 0x02) => self.op_8xy2(a, b),
      (0x08, _, _, 0x03) => self.op_8xy3(a, b),
      (0x08, _, _, 0x04) => self.op_8xy4(a, b),
      (0x08, _, _, 0x05) => self.op_8xy5(a, b),
      (0x08, _, _, 0x06) => self.op_8xy6(a),
      (0x08, _, _, 0x07) => self.op_8xy7(a, b),
      (0x08, _, _, 0xE) => self.op_8xye(a),
      (0x09, _, _, 0x0) => self.op_9xy0(a, b),
      (0x0A, _, _, _) => self.op_annn(nnn),
      (0x0B, _, _, _) => self.op_bnnn(nnn),
      (0x0C, _, _, _) => self.op_cxkk(a, kk),
      (0x0D, _, _, _) => self.op_dxyn(a, b, c),
      (0x0E, _, 0x09, 0x0E) => self.op_ex9e(a),
      (0x0E, _, 0x0A, 0x01) => self.op_exa1(a),
      (0x0F, _, 0x00, 0x07) => self.op_fx07(a),
      (0x0F, _, 0x00, 0x0A) => self.op_fx0a(a),
      (0x0F, _, 0x01, 0x05) => self.op_fx15(a),
      (0x0F, _, 0x01, 0x08) => self.op_fx18(a),
      (0x0F, _, 0x01, 0x0E) => self.op_fx1e(a),
      (0x0F, _, 0x02, 0x09) => self.op_fx29(a),
      (0x0F, _, 0x03, 0x03) => self.op_fx33(a),
      (0x0F, _, 0x05, 0x05) => self.op_fx55(a),
      (0x0F, _, 0x06, 0x05) => self.op_fx65(a),
      _ => panic!("Couldn't find a matching opcode. Panic: {:X}", a),
    }
    // self.pc = self.pc + 2;
  }

  // calls program at adress nnn
  pub fn op_0nnn(&mut self, adr: usize) {
    // println!("0nnn - SYS addr");
    self.pc = adr;
    self.next_instruction();
  }

  //Clears the screen
  pub fn op_00e0(&mut self) {
    // println!("00E0 - CLS");
    self.vram = [[0; VRAMW]; VRAMH];
    self.update_screen = true;
    self.next_instruction();
  }

  //return from subroutine
  pub fn op_00ee(&mut self) {
    // println!("00EE - RET");
    self.sp = self.sp - 1;
    self.pc = self.stack[self.sp] as usize;
    self.next_instruction();
    println!("PC: 0x{:x}", self.pc)
  }

  //jumps to an address
  pub fn op_1nnn(&mut self, adr: usize) {
    // println!("1nnn - JP addr");
    self.pc = adr;
  }

  //calls subroutine
  pub fn op_2nnn(&mut self, adr: usize) {
    // println!("2nnn - CALL addr");
    // println!("stack {:?}", self.stack);
    // println!("sp {:?}", self.sp);
    self.stack[self.sp] = self.pc as u16;
    self.sp = self.sp + 1;
    self.pc = adr;
  }
  //Skip next introduction if vx == kk
  pub fn op_3xkk(&mut self, x: usize, kk: u8) {
    // println!("3xkk - SE Vx, byte");
    if self.v[x] == kk {
      self.next_instruction();
      self.next_instruction();
    }
    self.next_instruction();
    println!("PC after 3xkk: {:X}", self.pc);
  }

  //Skip next introduction if vx != kk
  pub fn op_4xkk(&mut self, x: usize, kk: u8) {
    // println!("4xkk - SNE Vx, byte");
    if self.v[x] != kk {
      self.next_instruction();
      self.next_instruction();
    }
    self.next_instruction();
  }
  //Skip next introduction if Vx == Vy
  pub fn op_5xy0(&mut self, x: usize, y: usize) {
    // println!("5xy0 - SE Vx, Vy");
    if self.v[x] == self.v[y] {
      self.next_instruction();
      self.next_instruction();
    }
    self.next_instruction();
  }

  //Sets VX to NN -> Vx = NN
  pub fn op_6xnn(&mut self, x: usize, kk: u8) {
    // println!("6xkk - LD Vx, byte");
    self.v[x] = kk;
    self.next_instruction();
  }
  //adds value vx + kk and store in in vx
  pub fn op_7xkk(&mut self, x: usize, kk: u8) {
    // println!("7xkk - ADD Vx, byte");
    self.v[x] = self.v[x].wrapping_add(kk);
    self.next_instruction();
  }

  //set Vx = Vy.
  pub fn op_8xy0(&mut self, x: usize, y: usize) {
    // println!("8xy0 - LD Vx, Vy");
    self.v[x] = self.v[y];
    self.next_instruction();
  }
  //set vx = vx or vy
  pub fn op_8xy1(&mut self, x: usize, y: usize) {
    // println!("8xy1 - OR Vx, Vy");
    self.v[x] = self.v[x] | self.v[y];
    self.next_instruction();
  }
  //set vx = vx and vy
  pub fn op_8xy2(&mut self, x: usize, y: usize) {
    // println!("8xy2 - AND Vx, Vy");
    self.v[x] = self.v[x] & self.v[y];
    self.next_instruction();
  }
  //set vx = vx xor vy
  pub fn op_8xy3(&mut self, x: usize, y: usize) {
    // println!("8xy3 - XOR Vx, Vy");
    self.v[x] = self.v[x] ^ self.v[y];
    self.next_instruction();
  }

  //set vx = vx + vy, set carry
  pub fn op_8xy4(&mut self, x: usize, y: usize) {
    // println!("8xy4 - ADD Vx, Vy");
    let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
    self.v[x] = sum;
    self.v[0xF] = carry as u8;
    self.next_instruction();
  }

  //set vx = vx - vy, set carry = not borrow
  pub fn op_8xy5(&mut self, x: usize, y: usize) {
    // println!("8xy5 - SUB Vx, Vy");
    if self.v[x] > self.v[y] {
      self.v[0xF] = 1;
    } else {
      self.v[0xF] = 0;
    }

    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    self.next_instruction();
  }
  //Set Vx = Vx SHR 1
  pub fn op_8xy6(&mut self, x: usize) {
    // println!("8xy6 - SHR Vx , Vy");
    self.v[0xf] = self.v[x] & 1;

    self.v[x] = self.v[x] / 2;
    self.next_instruction();
  }
  //Set Vx = Vy - Vx, set VF = NOT borrow.
  pub fn op_8xy7(&mut self, x: usize, y: usize) {
    // println!("8xy7 - SUBN Vx, Vy");
    if self.v[y] > self.v[x] {
      self.v[0xF] = 1;
    } else {
      self.v[0xF] = 0;
    }

    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    self.next_instruction();
  }
  //Set Vx = Vx SHL 1.
  pub fn op_8xye(&mut self, x: usize) {
    // println!("8xyE - SHL Vx , Vy");
    self.v[0xf] = (self.v[x] >> 7) & 1;
    self.v[x] = self.v[x] * 2;
    self.next_instruction();
  }

  //Skip next instruction if Vx != Vy.
  pub fn op_9xy0(&mut self, x: usize, y: usize) {
    //println!("9xy0 - SNE Vx, Vy");
    if self.v[x] != self.v[y] {
      self.next_instruction();
    }
  }
  //Set I = nnn.
  pub fn op_annn(&mut self, adr: usize) {
    //  println!("Annn - LD I, addr");
    self.i = adr;
    self.next_instruction();
  }
  //Jump to location nnn + V0.
  pub fn op_bnnn(&mut self, adr: usize) {
    // println!("Bnnn - JP V0, addr");
    self.pc = adr + self.v[0] as usize;
  }
  //Set Vx = random byte AND kk.
  pub fn op_cxkk(&mut self, x: usize, adr: u8) {
    // println!("Cxkk - RND Vx, byte");
    let mut rng = rand::thread_rng();
    self.v[x] = rng.gen::<u8>() & adr;
    self.next_instruction();
  }
  //Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
  pub fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
    //println!("Dxyn - DRW Vx, Vy, nibble");
    self.v[0xf] = 0;
    for row in 0..n {
      let y = (self.v[y] as usize + row) % VRAMH;
      for bit in 0..8 {
        let x = (self.v[x] as usize + bit) % VRAMW;
        let val = ((self.memory[self.i + row] as usize >> (7 - bit)) & 1) as u8;
        self.v[0xf] |= val & self.vram[y][x];
        self.vram[y][x] ^= val;
      }
    }
    self.update_screen = true;
    self.next_instruction();
  }
  //Skip next instruction if key with the value of Vx is pressed.
  pub fn op_ex9e(&mut self, x: usize) {
    // println!("Ex9E - SKP Vx");
    if self.keyboard[self.v[x] as usize] {
      self.next_instruction();
    }
  }
  //Skip next instruction if key with the value of Vx is not pressed.
  pub fn op_exa1(&mut self, x: usize) {
    // println!("ExA1 - SKNP Vx");
    if !self.keyboard[self.v[x] as usize] {
      self.next_instruction();
    }
  }

  //Set Vx = delay timer value.
  pub fn op_fx07(&mut self, x: usize) {
    // println!("Fx07 - LD Vx, DT");
    self.v[x] = self.delay;
    self.next_instruction();
  }

  //Wait for a key press, store the value of the key in Vx.
  pub fn op_fx0a(&mut self, x: usize) {
    println!("Fx0A - LD Vx, K");

    for i in 0..self.keyboard.len() {
      if self.keyboard[i] {
        //self.keypad_waiting = false;
        self.v[x] = i as u8;
        self.next_instruction();
        break;
      }
    }
  }
  //Set delay timer = Vx.
  pub fn op_fx15(&mut self, x: usize) {
    // println!("Fx15 - LD DT, Vx");
    self.delay = self.v[x];
    self.next_instruction();
  }

  //Set sound timer = Vx.
  pub fn op_fx18(&mut self, x: usize) {
    // println!("Fx18 - LD ST, Vx");
    self.sound = self.v[x];
    self.next_instruction();
  }

  //Set I = I + Vx.
  pub fn op_fx1e(&mut self, x: usize) {
    // println!("Fx1E - ADD I, Vx");
    self.i = self.i + (self.v[x] as usize);
    self.next_instruction();
  }

  //Set I = location of sprite for digit Vx.
  pub fn op_fx29(&mut self, x: usize) {
    // println!("Fx29 - LD F, Vx");
    //because they are 5 bytes long
    self.i = (self.v[x] as usize) * 5;
    self.next_instruction();
  }
  // Store BCD representation of Vx in memory locations I, I+1, and I+2.
  pub fn op_fx33(&mut self, x: usize) {
    // println!("Fx33 - LD B, Vx");
    // X23 -> x in i
    self.memory[self.i] = self.v[x] / 100;
    // 1X3 -> x in i+1
    self.memory[self.i + 1] = (self.v[x] % 100) / 10;
    // 12X -> x in i+2
    self.memory[self.i + 2] = self.v[x] % 10;
    self.next_instruction();
  }

  //Store registers V0 through Vx in memory starting at location I.
  pub fn op_fx55(&mut self, x: usize) {
    // println!("Fx55 - LD [I], Vx");
    self.memory[self.i..self.i + x + 1].copy_from_slice(&self.v[0..x + 1]);
    self.next_instruction();
  }
  //Read registers V0 through Vx from memory starting at location I.
  fn op_fx65(&mut self, x: usize) {
    // println!("Fx65 - LD Vx, [I]");
    self.v[0..x + 1].copy_from_slice(&self.memory[self.i..self.i + x + 1]);
    self.next_instruction();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_op00ee() {}
}
