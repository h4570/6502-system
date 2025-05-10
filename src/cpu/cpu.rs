use super::instructions::{brk, lda};
use super::{memory::Memory, registers::Registers}; // zakładając, że masz moduł brk

pub struct Cpu {
    pub(super) registers: Registers,
    pub(super) memory: Memory,
    pub(super) instructions: [fn(&mut Cpu) -> u8; 256],
    pub(super) exit: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            registers: Registers { a: 0, pc: 0 },
            memory: Memory { data: [0; 65536] },
            instructions: [Cpu::unknown_opcode; 256],
            exit: false,
        };

        cpu.instructions[0xA9] = lda::lda_0xa9;
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

        let low = self.memory.data[0xFFFC] as u16;
        let high = self.memory.data[0xFFFD] as u16;
        self.registers.pc = (high << 8) | low;
    }

    pub fn load_program(&mut self, program: &[u8], addr: u16) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory.data[addr as usize + i] = byte;
        }
        self.registers.pc = addr;
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch_byte();

            if opcode as usize > self.instructions.len() {
                panic!("Out of bounds opcode! {:#04x}", opcode);
            }

            let instruction = self.instructions[opcode as usize];
            let _cycles: u8 = instruction(self);

            if self.exit {
                break;
            }
        }
    }
}
