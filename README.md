# ISA Benchmarks

Measuring the instruction count of different ISAs.

## Usage

```
nix develop
just run fibonacci
just run iter_sha
just run iter_keccak
```

## Results

Here are the measurements of the number of QEMU instructions per benchmark and target:

### Fibonacci

|Target|Intructions|
|------|---------|
|x86_64| 1535423|
|riscv64| 5277796|
|aarch64| 5212439|

### Iterative SHA

|Target|Intructions|
|------|---------|
|x86_64| 2569565|
|riscv64| 42894630|
|aarch64| 42894630|

### Iterative Keccak

|Target|Intructions|
|------|---------|
|x86_64| 62359535|
|riscv64| 67244706|
|aarch64| 39524653|
