use super::{flags::Flags, instruction_table::arrange_instruction_table, registers::Registers};
use crate::mem::ram::Ram;

pub struct Cpu {
    pub(super) registers: Registers,
    pub(super) memory: Ram,
    pub(super) flags: Flags,
    pub(super) instructions: [fn(&mut Cpu) -> u8; 256],
    pub(super) exit: bool,
}

impl Cpu {
    pub fn new(ram: Ram) -> Self {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                pc: 0,
                sp: 0,
            },
            flags: Flags {
                n: 0,
                z: 0,
                c: 0,
                i: 0,
                d: 0,
                v: 0,
            },
            memory: ram,
            instructions: [Cpu::unknown_opcode; 256],
            exit: false,
        };

        arrange_instruction_table(&mut cpu.instructions);

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
