# C & Rust FFI Example

This is a simple example of how to use C and Rust together using FFI.

## Introduction

1. Compile C code ([lib](./lib)) into a static library using the `cmake` crate.
2. Generate Rust bindings using `bindgen` from the C header file ([lib/lib.h](./lib/lib.h)).
3. Build the Rust code ([src](./src)) and link to the C library using `cargo`.

## Prerequisites

- Cargo 1.70 or higher
- CMake 3.15 or higher

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```
