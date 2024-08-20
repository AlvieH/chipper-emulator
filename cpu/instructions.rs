use crate::cpu::cpu_data::*;

pub fn translate_opcode(opcode_bytes : &u8) -> Opcode {
    return Opcode {
        instruction : "Unimplemented".to_string(),
        instruction_type : InstructionType::UnknownInstructionType,
        addressing_mode : AddressingMode::UnknownAddressingMode,
        source_register : '0',
        destination_register : '0',
        extra_info : 0
    };
}

// Extracts the command from a raw opcode
// Equal to (opcode & 0b11110000)
fn opcode_command(opcode_bytes : u8) -> u8 {
    return opcode_bytes & TOP_4_BITS;
}