use super::instructions::{brk, lda};
use crate::cpu::cpu::Cpu;

// TODO:
// Podstawowe instrukcje transferu danych
//   LDX/LDY - Ładowanie do rejestrów X i Y
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
    // BRK
    cpu_instructions[0x00] = brk::brk_0x00;
}
