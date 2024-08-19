struct Cpu {
    pub program_counter : u16,
    pub stack_pointer : u8,
    pub accumulator : u8,
    pub register_x : u8,
    pub register_y : u8,
    pub status : u8,
    memory_size : u16,
    memory = [u8; memory_size as usize],
}

fn build_cpu 