# ISA Benchmarks

Measuring the instruction count of different ISAs.

## Usage

```
nix develop
just run fibonacci
just run iter_sha
just run iter_keccak
just run eth_block
```

For now, the instruction measurement only works on x86 Linux due to a binary dependency on the QEMU plugin.

To generate detailed breakdown of WASM instructions use:
```
just run-wasm32-trace eth_block
```

For now, WASM tracing only works on x86 Linux due to a binary dependency on patched wasm-interp.

## Results

Here are the measurements of the number of QEMU instructions per benchmark and target:
- x86_64: Rustc to `x86_64-unknown-linux-gnu` with -sha,-ssse3,-sse4.1,-avx2
- risc64: Rustc to `riscv64gc-unknown-linux-gnu` with +zknh
- aarch64: Rustc to `aarch64-unknown-linux-gnu` with -sha3,-sve2-sha3
- wasm32+clang: Rustc to `wasm32-wasip1`, then w2c2 to C, then Clang to x86_64
- wasm32+zig:   Rustc to `wasm32-wasip1`, then w2c2 to C, then Zig to x86_64
- wasm32+eval:  Rustc to `wasm32-wasip1`, then run Wasmer interpreter with cost 1 for each WASM instruction
- wasm32+costs: As `wasm32-eval`, but `get_local`, `const.i32` and `const.i64` are free

The folder `reports/` contains detailed breakdown of WASM instructions for each benchmark.

### Fibonacci

|Target      |Intructions|
|------      |---------  |
|x86_64      | 1535423   |
|riscv64     | 5277796   |
|aarch64     | 5212439   |
|wasm32+clang| 1318539   |
|wasm32+zig  | 1535433   |
|wasm32+eval | 4004505   |
|wasm32+costs| 1626717  |

### Iterative SHA

|Target      |Intructions|
|-------     |---------  |
|x86_64      |  2569565  |
|riscv64     | 42894630  |
|aarch64     | 20084604  |
|wasm32+clang| 37244819  |
|wasm32+zig  | 30914079  |
|wasm32+eval | 55683326  |
|wasm32+costs| 25103699  |

### Iterative Keccak

|Target      |Intructions|
|-------     |---------  |
|x86_64      | 62359535  |
|riscv64     | 67244706  |
|aarch64     | 39524653  |
|wasm32+clang| 67784936  |
|wasm32+zig  | 50354196  |
|wasm32+eval |105243525  |
|wasm32+costs| 39223788  |


### Ethereum block

|Target      |Intructions|
|------      |---------  |
|x86_64      | 255674553 |
|riscv64     | 320867198 |
|aarch64     | 191864567 |
|wasm32+clang| 410921688 |
|wasm32+zig  | 376863421 |
|wasm32+eval | 607824971 |
|wasm32+costs| 237136572 |
