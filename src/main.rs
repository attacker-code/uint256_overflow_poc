mod syscall;

use syscall::{Uint256MulSyscall, SyscallContext};
use std::panic::AssertUnwindSafe;

fn main() {
    let mut ctx = SyscallContext { clk: 0 };
    let syscall = Uint256MulSyscall;

    // Very large pointers cause overflow
    let x_ptr: u32 = 0xFFFF_FFF0;
    let y_ptr: u32 = 1;

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        syscall.emulate(&mut ctx, x_ptr, y_ptr)
    }));

    match result {
        Ok(_) => println!("Syscall completed without panic."),
        Err(_) => println!("Panic detected due to modulus_ptr overflow!"),
    }
}
