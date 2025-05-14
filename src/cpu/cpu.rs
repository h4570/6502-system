use super::{flags::Flags, instruction_table::arrange_instruction_table, registers::Registers};
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

    /// Returns processor status byte combining all flags
    pub fn get_processor_status(&self) -> u8 {
        (self.flags.n << 7)     // Negative flag (bit 7)
            | (self.flags.v << 6)    // Overflow flag (bit 6)
            | (1 << 5)               // Unused bit, always set to 1 (bit 5)
            | (self.flags.b << 4)    // Break flag (bit 4)
            | (self.flags.d << 3)    // Decimal mode flag (bit 3)
            | (self.flags.i << 2)    // Interrupt disable flag (bit 2)
            | (self.flags.z << 1)    // Zero flag (bit 1)
            | self.flags.c // Carry flag (bit 0)
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

    /// Prints current CPU state for debugging
    pub fn trace_state(&self) -> String {
        format!(
            "A:{:02X} X:{:02X} Y:{:02X} PC:{:04X} SP:{:02X} | N:{} V:{} B:{} D:{} I:{} Z:{} C:{}",
            self.registers.a,
            self.registers.x,
            self.registers.y,
            self.registers.pc,
            self.registers.sp,
            self.flags.n,
            self.flags.v,
            self.flags.b,
            self.flags.d,
            self.flags.i,
            self.flags.z,
            self.flags.c
        )
    }

    pub fn endless_run(&mut self) {
        let max_cycles: u64 = 10000 * 10; // Approx 10000 iterations
        debug!("CPU starting execution at PC={:#06X}", self.registers.pc);
        debug!("Initial state: {}", self.trace_state());

        let total_cycles = self.step(max_cycles);

        debug!("CPU halted: executed {} cycles", total_cycles);
        debug!("Final state: {}", self.trace_state());
    }

    pub fn run(&mut self, cycles_to_run: u64) -> u64 {
        debug!(
            "CPU running for {} cycles from PC={:#06X}",
            cycles_to_run, self.registers.pc
        );
        debug!("Initial state: {}", self.trace_state());

        let cycles_executed = self.step(cycles_to_run);

        debug!("CPU executed {} cycles", cycles_executed);
        debug!("Final state: {}", self.trace_state());

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
