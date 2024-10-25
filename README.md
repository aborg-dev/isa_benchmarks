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

## Results

Here are the measurements of the number of QEMU instructions per benchmark and target:

### Fibonacci

|Target|Intructions|
|------|---------|
|x86_64|  1535423|
|riscv64| 5277796|
|aarch64| 5212439|
|wasm32+clang|  1318539|
|wasm32+zig|  1535433|

### Iterative SHA

|Target|Intructions|
|-------|---------|
|x86_64 |  2569565|
|riscv64| 42894630|
|aarch64| 20084604|
|wasm32+clang| 37244819|
|wasm32+zig| 30914079|

### Iterative Keccak

|Target|Intructions|
|-------|---------|
|x86_64 | 62359535|
|riscv64| 67244706|
|aarch64| 39524653|
|wasm32+clang| 67784936|
|wasm32+zig| 50354196|

### Ethereum block

|Target|Intructions|
|------|---------|
|x86_64| 255674553|
|riscv64|320867198|
|aarch64|191864567|
|wasm32+clang|410921688|
|wasm32+zig|376863421|
