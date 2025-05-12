# 6502 Computer System Emulator

This project is an emulator for a hypothetical computer system based on the classic 6502 CPU with custom GPU and DMA controller.

## System Specifications

### Memory
- 64KB of RAM available to the system

### Data Bus
- Purely conceptual in the implementation (simplified)
- Supports two devices simultaneously, allowing DMA transfers without blocking CPU operation

### CPU (6502)
- Classic 6502 instruction set
- Clock speed: 4MHz
- Full emulation of all original 6502 operations and addressing modes

### GPU
- Clock frequency: 4MHz
- Resolution: 256×240 pixels
- Color depth: 4bpp (16 colors using VGA palette)
- VRAM: 64KB
- Supports double buffering
- Buffer size: 30.7KB per frame buffer
- Data transfer via DMA controller

#### GPU Registers (memory-mapped to 6502 address space)
- Control Register: Enable/disable GPU, color mode selection
- Active Buffer Register: Buffer selection (0 = first buffer, 1 = second buffer)
- Status Register: VSYNC, HSYNC, and other status flags
- Interrupt Register: Enable/disable VSYNC/HSYNC interrupts

### DMA Controller
- Clock frequency: 2MHz
- Transfers data from 6502 memory to GPU VRAM
- Operates without halting the CPU (dual-device support on the data bus)
- Transfer rate: 1 byte per cycle

#### DMA Registers (memory-mapped to 6502 address space)
- Source Address Register: Memory location to read from
- Destination Address Register: VRAM location to write to
- Transfer Length Register: Number of bytes to transfer
- Control Register: Start/stop operations, transfer direction, status flags

## Usage

[Usage instructions and examples will be added as the project develops]

## Build Instructions

[Build instructions will be added as the project develops]

## Project Status

This emulator is currently under development. See the TODO list for upcoming features and improvements.

## License

[License information]
