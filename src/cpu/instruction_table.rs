use super::instructions::{brk, lda, ldx, ldy};
use crate::cpu::cpu::Cpu;

// TODO:
// Podstawowe instrukcje transferu danych
//   STA/STX/STY - Zapisywanie wartości z rejestrów do pamięci
//   TAX/TAY/TXA/TYA - Transfer między rejestrami
// Operacje arytmetyczne i logiczne
//   ADC/SBC - Dodawanie/odejmowanie z przenoszeniem
//   AND/ORA/EOR - Operacje logiczne (AND, OR, XOR)
//   INC/DEC - Inkrementacja/dekrementacja pamięci
//   INX/INY/DEX/DEY - Inkrementacja/dekrementacja rejestrów
// Instrukcje porównania i skoków
//   CMP/CPX/CPY - Porównanie
//   JMP - Skok bezwarunkowy
//   BEQ/BNE/BCS/BCC/BMI/BPL - Skoki warunkowe
// Dodatkowe operacje
//   PHA/PLA - Operacje na stosie
//   JSR/RTS - Podprogramy

pub(crate) fn arrange_instruction_table(cpu_instructions: &mut [fn(&mut Cpu) -> u8; 256]) {
    // LDA
    cpu_instructions[0xA9] = lda::lda_0xa9;
    cpu_instructions[0xA5] = lda::lda_0xa5;
    cpu_instructions[0xB5] = lda::lda_0xb5;
    cpu_instructions[0xAD] = lda::lda_0xad;
    cpu_instructions[0xBD] = lda::lda_0xbd;
    cpu_instructions[0xB9] = lda::lda_0xb9;
    cpu_instructions[0xA1] = lda::lda_0xa1;
    cpu_instructions[0xB1] = lda::lda_0xb1;
    // LDX
    cpu_instructions[0xA2] = ldx::ldx_0xa2;
    cpu_instructions[0xA6] = ldx::ldx_0xa6;
    cpu_instructions[0xB6] = ldx::ldx_0xb6;
    cpu_instructions[0xAE] = ldx::ldx_0xae;
    cpu_instructions[0xBE] = ldx::ldx_0xbe;
    // LDY
    cpu_instructions[0xA0] = ldy::ldy_0xa0;
    cpu_instructions[0xA4] = ldy::ldy_0xa4;
    cpu_instructions[0xB4] = ldy::ldy_0xb4;
    cpu_instructions[0xAC] = ldy::ldy_0xac;
    cpu_instructions[0xBC] = ldy::ldy_0xbc;
    // BRK
    cpu_instructions[0x00] = brk::brk_0x00;
}
