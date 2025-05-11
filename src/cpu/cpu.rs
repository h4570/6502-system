use super::instructions::{brk, lda};
use super::{flags::Flags, memory::Memory, registers::Registers}; // zakładając, że masz moduł brk

pub struct Cpu {
    pub(super) registers: Registers,
    pub(super) memory: Memory,
    pub(super) flags: Flags,
    pub(super) instructions: [fn(&mut Cpu) -> u8; 256],
    pub(super) exit: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                pc: 0,
            },
            flags: Flags { z: 0, n: 0 },
            memory: Memory { data: [0; 65536] },
            instructions: [Cpu::unknown_opcode; 256],
            exit: false,
        };

        // LDA
        cpu.instructions[0xA9] = lda::lda_0xa9;
        cpu.instructions[0xA5] = lda::lda_0xa5;
        cpu.instructions[0xB5] = lda::lda_0xb5;
        cpu.instructions[0xAD] = lda::lda_0xad;
        cpu.instructions[0xBD] = lda::lda_0xbd;
        cpu.instructions[0xB9] = lda::lda_0xb9;
        cpu.instructions[0xA1] = lda::lda_0xa1;
        cpu.instructions[0xB1] = lda::lda_0xb1;
        // BRK
        cpu.instructions[0x00] = brk::brk_0x00;

        cpu
    }

    fn unknown_opcode(&mut self) -> u8 {
        panic!(
            "Unknown opcode: {:#04x}",
            self.memory.data[self.registers.pc as usize - 1]
        );
    }

    pub(super) fn fetch_byte(&mut self) -> u8 {
        let val = self.memory.data[self.registers.pc as usize];
        self.registers.pc += 1;
        return val;
    }

    pub fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.x = 0;
        self.registers.y = 0;

        let lo = self.memory.data[0xFFFC] as u16;
        let hi = self.memory.data[0xFFFD] as u16;
        self.registers.pc = (hi << 8) | lo;
    }

    pub fn load_program(&mut self, program: &[u8], addr: u16) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory.data[addr as usize + i] = byte;
        }
        self.registers.pc = addr;
    }

    pub fn run(&mut self) {
        let mut _total_cycles: u64 = 0;

        loop {
            let opcode = self.fetch_byte();

            if opcode as usize > self.instructions.len() {
                panic!("Out of bounds opcode! {:#04x}", opcode);
            }

            let instruction = self.instructions[opcode as usize];
            let cycles: u8 = instruction(self);

            _total_cycles += cycles as u64;

            if self.exit {
                break;
            }
        }
    }
}
