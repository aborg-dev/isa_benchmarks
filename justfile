plugins := "-plugin /home/polygon/repos/qemu/build/tests/tcg/plugins/libinsn.so -d plugin"

build benchmark:
  @echo "Building {{benchmark}}"
  cargo build --bin {{benchmark}} --target x86_64-unknown-linux-gnu --release
  cargo build --bin {{benchmark}} --target riscv64gc-unknown-linux-gnu --release
  cargo build --bin {{benchmark}} --target aarch64-unknown-linux-gnu --release

bench benchmark: (build benchmark)
  @echo "Benchmarking {{benchmark}}"
  qemu-x86_64 {{plugins}} target/x86_64-unknown-linux-gnu/release/{{benchmark}}
  qemu-riscv64 {{plugins}} target/riscv64gc-unknown-linux-gnu/release/{{benchmark}}
  qemu-aarch64 {{plugins}} target/aarch64-unknown-linux-gnu/release/{{benchmark}}
