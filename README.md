# MEME-CONNECT, old game with more tricks and endless mode

_*Important notice:*_ Work in progress, expect broken features. For now, just basic things implemented.

<img width="1673" alt="image" src="https://github.com/user-attachments/assets/5aaa930e-6b20-4ffd-94b8-3e5133b538a3" />

## Requirements:

Main requirement is Rust toolchain. And for optimized binary, external linker is used instead of default one. Refer to instruction below for each compile platform.

### Linux

Install `clang` and `llvm`

### Windows

Install `llvm`

### Mac

Nothing is required

## Running

### Natively

```shell
cargo run
```

### As web app (Linux and Mac only, no script for Windows yet)

```shell
bash serve_wasm.sh
```
