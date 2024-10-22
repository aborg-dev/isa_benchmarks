plugins := "-plugin deps/libinsn.so -d plugin"

build-x86 $RUSTFLAGS="-C target-feature=-sha,-ssse3,-sse4.1,-avx2":
  cargo build --target x86_64-unknown-linux-gnu --release

build-riscv64 $RUSTFLAGS="-C target-feature=+zknh":
  cargo build --target riscv64gc-unknown-linux-gnu --release

build-aarch64 $RUSTFLAGS="-C target-feature=-sha3,-sve2-sha3":
  cargo build --target aarch64-unknown-linux-gnu --release

build: build-x86 build-riscv64 build-aarch64

run benchmark: build
  @echo "Benchmarking {{benchmark}}"
  qemu-x86_64 {{plugins}} target/x86_64-unknown-linux-gnu/release/{{benchmark}}
  qemu-riscv64 {{plugins}} target/riscv64gc-unknown-linux-gnu/release/{{benchmark}}
  qemu-aarch64 {{plugins}} target/aarch64-unknown-linux-gnu/release/{{benchmark}}
