use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;
use crate::trace_instruction;

pub(crate) fn bvs_0x70(cpu: &mut Cpu) -> u8 {
    // Branch if overflow flag is set (V=1)
    let cycles = branch_if(cpu, cpu.flags.v == 1, "BVS");
    trace_instruction!(cpu, "BVS", "0x70", "Relative");
    cycles
}
