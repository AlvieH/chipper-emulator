use std::Boxed;
use cpu_const::*;
// Generic CPU - expandable memory that's set on initialization
struct Cpu {
    pub program_counter : u16,
    pub stack_pointer : u8,
    pub accumulator : u8,
    pub register_x : u8,
    pub register_y : u8,
    pub status : u8,
    memory = [u8; MEMORY_SIZE as usize],
}

impl Cpu {
    fn boxed_cpu() -> Box<Cpu> {
        Box::new(Cpu);
    }
}