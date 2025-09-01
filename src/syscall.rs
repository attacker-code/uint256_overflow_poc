use num::{BigUint, One, Zero};

// Constant that simulates UINT256 NUM_WORDS
pub const UINT256_NUM_WORDS: usize = 8;
pub const WORD_SIZE: usize = 4;

pub struct Uint256MulSyscall;

pub struct SyscallContext {
    pub clk: u32,
}

impl Uint256MulSyscall {
    pub fn emulate(
        &self,
        ctx: &mut SyscallContext,
        arg1: u32,
        arg2: u32,
    ) -> Option<u32> {
        let clk = ctx.clk;

        let x_ptr = arg1;
        if !x_ptr.is_multiple_of(4) {
            panic!("x_ptr is not aligned");
        }
        let y_ptr = arg2;
        if !y_ptr.is_multiple_of(4) {
            panic!("y_ptr is not aligned");
        }

        // We calculate the module pointer
        let modulus_ptr = y_ptr.checked_add((UINT256_NUM_WORDS * WORD_SIZE) as u32)
            .expect("Overflow en modulus_ptr!");

        println!("x_ptr = {:#X}, y_ptr = {:#X}, modulus_ptr = {:#X}", x_ptr, y_ptr, modulus_ptr);

        // Simulation of the operation
        let uint256_x = BigUint::from(123u32);
        let uint256_y = BigUint::from(456u32);
        let uint256_modulus = BigUint::zero(); // Simulate zero

        let modulus = if uint256_modulus.is_zero() {
            BigUint::one() << 256
        } else {
            uint256_modulus
        };

        let result = (uint256_x * uint256_y) % modulus;

        println!("Resultado: {}", result);

        ctx.clk += 1;
        None
    }
}
