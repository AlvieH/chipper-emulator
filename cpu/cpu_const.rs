// Addressable memory space for NES
pub const MEMORY_SIZE : u16 = 65_535;

// NES Addressing modes
pub enum AddressingModes {
    // Source and destination is implied in the instruction.
    // No further operand needs to be specified.
    Implicit,
    // Instructions that operate directly on the accumulator.
    Accumulator,
    // Allows the processor to directly specify an 8 bit constant.
    Immediate,
    // Can only operate on the first 256 bytes of memory.
    ZeroPage,
    // Using zero-page addressing, access the memory location [Supplied + contents of X Register]
    ZeroPageX,
    // Using zero-page addressing, access the memory location [Supplied + contents of Y Register]
    ZeroPageY,
    // Used for branch instructions. Increment or decrement program counter by a supplied value.
    Relative,
    // Access a given 16-bit memory address.
    Absolute,
    // Access a 16-bit memory address at [Supplied + contents of X Register]
    AbsoluteX,
    // Access a 16-bit memory address at [Supplied + contents of Y Register]
    AbsoluteY,
    // Only used for JMP
    // TODO: Understand this addressing mode
    Indirect,
    // Usually used for accessing memory in the zero page.
    // TODO: Understand this addressing mode as well.
    IndexedIndirect,
    // Using an instruction containing the zero page location of the least significant byte of the desired address:
    // Add contents of Y register to find the actual target address.
    IndirectIndexed
}