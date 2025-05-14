use super::{
    flags::Flags, instruction_table::arrange_instruction_table, registers::Registers,
    utils::trace_state,
};
use crate::mem::ram::Ram;
use log::{debug, trace};

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
                b: 0,
            },
            memory: ram,
            instructions: [Cpu::unknown_opcode; 256],
            exit: false,
        };

        arrange_instruction_table(&mut cpu.instructions);

        cpu
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
        debug!(
            "Loading program of {} bytes at address {:#06X}",
            program.len(),
            addr
        );
        for (i, &byte) in program.iter().enumerate() {
            self.memory.data[addr as usize + i] = byte;
        }
        self.registers.pc = addr;
        debug!("Program loaded, PC set to {:#06X}", self.registers.pc);
    }

    /// Run with default limit of 10000 * 10 cycles (approx 10k iterations)
    pub fn endless_run(&mut self) {
        self.endless_run_limit(10000 * 10);
    }

    pub fn endless_run_limit(&mut self, max_cycles: u64) {
        debug!("CPU starting execution at PC={:#06X}", self.registers.pc);
        debug!("Initial state: {}", trace_state(self));

        let total_cycles = self.step(max_cycles);

        debug!("CPU halted: executed {} cycles", total_cycles);
        debug!("Final state: {}", trace_state(self));
    }

    pub fn run(&mut self, cycles_to_run: u64) -> u64 {
        debug!(
            "CPU running for {} cycles from PC={:#06X}",
            cycles_to_run, self.registers.pc
        );
        debug!("Initial state: {}", trace_state(self));

        let cycles_executed = self.step(cycles_to_run);

        debug!("CPU executed {} cycles", cycles_executed);
        debug!("Final state: {}", trace_state(self));

        cycles_executed
    }

    fn step(&mut self, cycles_to_run: u64) -> u64 {
        let mut cycles_executed: u64 = 0;
        let mut remaining_cycles: u8 = 0;

        while cycles_executed < cycles_to_run && !self.exit {
            // If we are not doing instruction currently, get new one.
            if remaining_cycles == 0 {
                let opcode = self.fetch_byte();
                trace!("Loading instruction at {:#04x}", opcode);
                let instruction = self.instructions[opcode as usize];
                remaining_cycles = instruction(self);
            }

            remaining_cycles -= 1;
            cycles_executed += 1;
        }

        cycles_executed
    }

    fn unknown_opcode(&mut self) -> u8 {
        panic!(
            "Tried to execute instruction at unknown opcode: {:#04x}",
            self.memory.data[self.registers.pc as usize - 1]
        );
    }
}
