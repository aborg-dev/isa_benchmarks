#!/bin/bash

BENCHMARK=iterative_sha
PLUGINS="-plugin /home/polygon/repos/qemu/build/tests/tcg/plugins/libinsn.so -d plugin"

cargo build --bin $BENCHMARK --target x86_64-unknown-linux-gnu --release
cargo build --bin $BENCHMARK --target riscv64gc-unknown-linux-gnu --release
cargo build --bin $BENCHMARK --target aarch64-unknown-linux-gnu --release

echo "x86_64"
qemu-x86_64 $PLUGINS target/x86_64-unknown-linux-gnu/release/$BENCHMARK
echo "riscv64gc"
qemu-riscv64 $PLUGINS target/riscv64gc-unknown-linux-gnu/release/$BENCHMARK
echo "aarch64"
qemu-aarch64 $PLUGINS target/aarch64-unknown-linux-gnu/release/$BENCHMARK
