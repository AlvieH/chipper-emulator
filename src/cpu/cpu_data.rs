// Addressable memory space for NES
pub const MEMORY_SIZE : u16 = 65_535;

// Status flags
pub const STATUS_DEFAULT : u8           = 0b0010_0000; // Bit 5 should always be 1
pub const STATUS_NEGATIVE : u8          = 0b1000_0000;
pub const STATUS_OVERFLOW : u8          = 0b0100_0000;
pub const STATUS_DECIMAL : u8           = 0b0000_1000;
pub const STATUS_INTERRUPT_DISABLE : u8 = 0b0000_0100;
pub const STATUS_ZERO : u8              = 0b0000_0010;
pub const STATUS_CARRY : u8             = 0b0000_0001;

// Used for logical operations
// NES instructions use a max of 3 bytes, with the opcode always being the most significant byte.
pub const BOTTOM_BYTE : u8 = 0xFF;
pub const MIDDLE_BYTE : u16 = 0xFF00;
pub const TOP_BYTE : u32 = 0xFF0000;



// Breaks the instruction into basic logical groups
pub enum InstructionType {
    LoadStore,
    RegisterTransfer,
    StackOperation,
    Logical,
    Arithmetic,
    IncrementDecrement,
    Shift,
    JumpCall,
    Branch,
    StatusFlag,
    SystemFunction,
    UnknownInstructionType // Should not be used.
}

// NES Addressing modes
pub enum AddressingMode {
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
    IndirectX,
    // Using an instruction containing the zero page location of the least significant byte of the desired address:
    // Add contents of Y register to find the actual target address.
    IndirectY,
    // Should never be used.
    UnknownAddressingMode
}

// Valid locations for operations to load from and store to
pub enum LoadStoreLocation {
    RegisterX,
    RegisterY, 
    Accumulator,
    ProgramCounter,
    StackPointer,
    Status,
    Memory,
    // Used for instructions that don't load or store any data.
    NoLoadStore
}

// Represents a single operation and its provided information.
pub struct Instruction<'a> {
    pub instruction : &'a str,
    pub instruction_type : InstructionType,
    pub addressing_mode : AddressingMode,
    pub source : LoadStoreLocation,
    pub destination : LoadStoreLocation,
    pub extra_info : u16,
    pub cycles : u8,
    pub length_bytes : u8
}

impl Instruction<'_> {
    pub const fn new<'a>(
        _instruction : &'a str,
        _instruction_type : InstructionType,
        _addressing_mode : AddressingMode,
        _source : LoadStoreLocation,
        _destination : LoadStoreLocation,
        _extra_info : u16,
        _cycles : u8,
        _length_bytes : u8
    ) -> Instruction<'a> {
        return Instruction {
            instruction : _instruction,
            instruction_type : _instruction_type,
            addressing_mode : _addressing_mode,
            source : _source,
            destination : _destination,
            extra_info : _extra_info,
            cycles : _cycles,
            length_bytes : _length_bytes
        };
    }
}
