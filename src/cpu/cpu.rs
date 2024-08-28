use crate::cpu::cpu_data::*;

// Generic CPU - expandable memory that's set on initialization
pub struct Cpu {
    pub program_counter : u16,
    pub stack_pointer : u8,
    pub accumulator : u8,
    pub register_x : u8,
    pub register_y : u8,
    pub status : u8,
    memory : [u8; MEMORY_SIZE as usize],
}

impl Cpu {
    // Returns a boxed NES CPU set to correct NES power-up state.
    pub fn boxed_cpu() -> Box<Cpu> {
        // Reference for CPU power-up state available here:
        // https://www.nesdev.org/wiki/CPU_power_up_state
        return Box::new(Cpu {
            program_counter : 0xFFFCu16,
            stack_pointer : 0xFDu8,
            accumulator : 0,
            register_x : 0,
            register_y : 0,
            status : STATUS_DEFAULT | STATUS_INTERRUPT_DISABLE,
            memory : [0; MEMORY_SIZE as usize]
        });
    }
}