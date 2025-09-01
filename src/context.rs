use std::collections::HashMap;

pub struct SyscallContext {
    pub clk: u32,
    pub memory: HashMap<u32, u32>,
}

impl SyscallContext {
    pub fn new() -> Self {
        Self {
            clk: 0,
            memory: HashMap::new(),
        }
    }

    // Safe memory read simulation
    pub fn mr_slice(&mut self, ptr: u32, len: usize) -> (Vec<u32>, Vec<u32>) {
        let modulus_ptr = ptr.checked_add(len as u32 * 4);
        match modulus_ptr {
            Some(_) => {
                let data = vec![0u32; len];
                (data.clone(), data)
            }
            None => panic!("Overflow detected in modulus_ptr!"),
        }
    }

    // Unsafe read simulation for x
    pub fn slice_unsafe(&self, ptr: u32, len: usize) -> Vec<u32> {
        vec![1u32; len] // we simulate that there is valid data
    }

    // Writing simulation
    pub fn mw_slice(&mut self, ptr: u32, data: &[u32]) -> Vec<u32> {
        for (i, word) in data.iter().enumerate() {
            self.memory.insert(ptr + i as u32 * 4, *word);
        }
        data.to_vec()
    }

    pub fn current_chunk(&self) -> u32 {
        0
    }

    pub fn postprocess(&self) -> Vec<u32> {
        vec![]
    }

    pub fn record_mut(&mut self) -> &mut Self {
        self
    }

    pub fn add_precompile_event(&mut self, _syscall_code: impl std::fmt::Debug, _syscall_event: impl std::fmt::Debug, _event: impl std::fmt::Debug) {}
}
