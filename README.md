# Uint256 Overflow PoC

This repository contains a **proof-of-concept (PoC)** for the `Uint256MulSyscall` vulnerability in a Rust-based emulator of EVM-like contracts.

The PoC demonstrates **panic due to unaligned pointers** and **modulus pointer overflow**, which can lead to a denial-of-service in the system.

---

## Project Structure

- `src/`
  - `main.rs`: Entry point to run the PoC.
  - `syscall.rs`: Contains the `Uint256MulSyscall` implementation.
- `Cargo.toml`: Rust project configuration.
- `lib/forge-std`: Optional Foundry standard library submodule (if you want to integrate Ethereum testing).

> Note: Some code was trimmed for clarity; only the essential syscall logic is included for demonstration purposes.

---

## Requirements

- Rust >= 1.70
- Cargo
- (Optional) Foundry toolkit if you want to integrate Ethereum testing: [https://book.getfoundry.sh](https://book.getfoundry.sh)

---

## Run the PoC

1. Clone the repository:

```bash
git clone git@github.com:attacker-code/uint256_overflow_poc.git
cd uint256_overflow_poc
````

2. Build the project:

```bash
cargo build
```

3. Run the PoC:

```bash
cargo run
```

You should see output similar to:

```
warning: unused variable: `clk`
  --> src/syscall.rs:20:13
...
thread 'main' panicked at src/syscall.rs:28:13:
y_ptr is not aligned
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Panic detected due to modulus_ptr overflow!
```

This confirms the **vulnerability in the syscall implementation**.

---

## Function Flow Affected

* `Uint256MulSyscall::emulate()`

  * Reads x and y from memory using potentially unsafe pointers
  * Computes modulus pointer without overflow checks
  * Performs multiplication modulo the modulus
  * Panics if pointers are misaligned or if overflow occurs

---

## Disclaimer

This PoC is for **educational and research purposes only**. Do **not deploy** to production.
