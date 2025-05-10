use crate::cpu::cpu::Cpu;

fn set_val(cpu: &mut Cpu, val: u8) {
    cpu.registers.a = val;

    cpu.flags.z = if val == 0 { 1 } else { 0 };
    cpu.flags.n = if (val & 128) != 0 { 1 } else { 0 };
}

// Immediate
pub(crate) fn lda_0xa9(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_byte();
    set_val(cpu, val);

    println!("Opcode: 0xA9");
    2
}

// Zeropage
pub(crate) fn lda_0xa5(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_byte();

    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    println!("Opcode: 0xA5");
    3
}

// Zeropage,X
pub(crate) fn lda_0xb5(cpu: &mut Cpu) -> u8 {
    let addr = (cpu.fetch_byte() + cpu.registers.x) & 0xFF; // z zawijaniem na stronę 0

    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    println!("Opcode: 0xB5");
    4
}

// Absolute
pub(crate) fn lda_0xad(cpu: &mut Cpu) -> u8 {
    let addr_first = cpu.fetch_byte() as u16;
    let addr_second = cpu.fetch_byte() as u16;
    let addr = (addr_second << 8) | addr_first;

    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    println!("Opcode: 0xAD");
    4
}

// Absolute,X
pub(crate) fn lda_0xbd(cpu: &mut Cpu) -> u8 {
    let addr_first = cpu.fetch_byte() as u16;
    let addr_second = cpu.fetch_byte() as u16;
    let addr_join = (addr_second << 8) | addr_first;
    let addr = addr_join + (cpu.registers.x as u16);

    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    println!("Opcode: 0xBD");
    if (addr_join & 0xFF00) != (addr & 0xFF00) {
        5
    } else {
        4
    }
}

// Absolute,Y
pub(crate) fn lda_0xb9(cpu: &mut Cpu) -> u8 {
    let addr_first = cpu.fetch_byte() as u16;
    let addr_second = cpu.fetch_byte() as u16;
    let addr_join = (addr_second << 8) | addr_first;
    let addr = addr_join + (cpu.registers.y as u16);

    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    println!("Opcode: 0xB9");
    if (addr_join & 0xFF00) != (addr & 0xFF00) {
        5
    } else {
        4
    }
}

// (Indirect,X)
pub(crate) fn lda_0xa1(cpu: &mut Cpu) -> u8 {
    let operand = cpu.fetch_byte() as u16;
    let addr1 = (operand + (cpu.registers.x as u16)) & 0xFF; // zawijanie w stronie zerowej

    let addr2_first = cpu.memory.data[addr1 as usize] as u16;
    let addr2_second = cpu.memory.data[(addr1 + 1) as usize] as u16;
    let addr2 = (addr2_second << 8) | addr2_first;

    let val = cpu.memory.data[addr2 as usize];
    set_val(cpu, val);

    println!("Opcode: 0xA1");
    6
}

// (Indirect,Y)
pub(crate) fn lda_0xb1(cpu: &mut Cpu) -> u8 {
    let addr1 = cpu.fetch_byte();

    let addr2_first = cpu.memory.data[addr1 as usize] as u16;
    let addr2_second: u16 = cpu.memory.data[((addr1 + 1) & 0xFF) as usize] as u16; // zawijanie w stronie zerowej
    let addr2_base = (addr2_second << 8) | addr2_first;
    let addr2 = addr2_base + (cpu.registers.y as u16);

    let val = cpu.memory.data[addr2 as usize];
    set_val(cpu, val);

    println!("Opcode: 0xB1");
    if (addr2_base & 0xFF00) != (addr2 & 0xFF00) {
        6
    } else {
        5
    }
}
