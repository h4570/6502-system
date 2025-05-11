mod mem;
use mem::ram::Ram;

mod cpu;
use cpu::cpu::Cpu;

fn main() {
    let mem = Ram::new();
    let mut cpu = Cpu::new(mem);
    let program = vec![0xA9, 0x42, 0x00];

    cpu.load_program(&program, 0);

    cpu.reset();

    cpu.run();

    println!("Emulator exit")
}
