# ISA Benchmarks

Measuring the instruction count of different ISAs.

## Usage

```
nix develop

cargo build --bin fibonacci  --target riscv64gc-unknown-linux-gnu --release
qemu-riscv64 \
    target/riscv64gc-unknown-linux-gnu/release/fibonacci

cargo build --bin iterative_sha  --target riscv64gc-unknown-linux-gnu --release
qemu-riscv64 \
    target/riscv64gc-unknown-linux-gnu/release/iterative_sha
```
