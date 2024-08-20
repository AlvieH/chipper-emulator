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
    pub fn boxed_cpu() -> Box<Cpu> {
        return Box::new(Cpu {
            program_counter : 0,
            stack_pointer : 0,
            accumulator : 0,
            register_x : 0,
            register_y : 0,
            status : 0,
            memory : [0; MEMORY_SIZE as usize]
        });
    }
}