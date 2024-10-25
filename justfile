plugins := "-plugin deps/libinsn.so -d plugin"

build-x86_64 $RUSTFLAGS="-C target-feature=-sha,-ssse3,-sse4.1,-avx2":
  cargo build --target x86_64-unknown-linux-gnu --release

build-riscv64 $RUSTFLAGS="-C target-feature=+zknh":
  cargo build --target riscv64gc-unknown-linux-gnu --release

build-aarch64 $RUSTFLAGS="-C target-feature=-sha3,-sve2-sha3":
  cargo build --target aarch64-unknown-linux-gnu --release

build-wasm:
  cargo build --target wasm32-wasip1 --release

run benchmark: \
  (run-x86_64 benchmark) \
  (run-riscv64 benchmark) \
  (run-aarch64 benchmark) \
  (run-wasm32 benchmark)

run-x86_64 benchmark: build-x86_64
  qemu-x86_64 {{plugins}} target/x86_64-unknown-linux-gnu/release/{{benchmark}}

run-riscv64 benchmark: build-riscv64
  qemu-riscv64 {{plugins}} target/riscv64gc-unknown-linux-gnu/release/{{benchmark}}

run-aarch64 benchmark: build-aarch64
  qemu-aarch64 {{plugins}} target/aarch64-unknown-linux-gnu/release/{{benchmark}}

run-wasm32 benchmark: build-wasm
  mkdir -p target/w2c2/{{benchmark}}
  cp target/wasm32-wasip1/release/{{benchmark}}.wasm target/w2c2/{{benchmark}}/module.wasm
  cp wasm/Makefile target/w2c2/{{benchmark}}
  cp wasm/main.c target/w2c2/{{benchmark}}
  (cd target/w2c2/{{benchmark}} && make)
  qemu-x86_64 {{plugins}} target/w2c2/{{benchmark}}/module
