pub struct CPU {
    pub a: u8, // ACC
    pub pc: u16,
    pub memory: [u8; 65536],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            pc: 0,
            memory: [0; 65536],
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;

        let low = self.memory[0xFFFC] as u16;
        let high = self.memory[0xFFFD] as u16;
        self.pc = (high << 8) | low;
    }

    pub fn load_program(&mut self, program: &[u8], addr: u16) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[addr as usize + i] = byte;
        }
        self.pc = addr;
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory[self.pc as usize];
            self.pc += 1;

            match opcode {
                0xA9 => {
                    // LDA
                    println!("Opcode: 0xA9");
                    let val = self.memory[self.pc as usize];
                    self.a = val;
                    self.pc += 1;
                }
                0x00 => {
                    // BRK
                    println!("Opcode: 0x00");
                    return;
                }
                _ => {
                    // Unknown
                    println!("Unknown opcode!");
                    return;
                }
            }
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    let program = vec![0xA9, 0x42, 0x00];

    cpu.load_program(&program, 0);

    cpu.reset();

    cpu.run();

    println!("Emulator exit")
}
