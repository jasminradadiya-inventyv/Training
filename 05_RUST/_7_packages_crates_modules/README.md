# Rust Packages, Crates, and Modules

This project demonstrates the organization of a Rust project using **Packages**, **Crates**, and **Modules**. It consolidates various Rust concepts from previous tasks into a single, modular codebase.

## Overview

The project illustrates how to structured code by separating functionality into distinct modules within a library folder (`all_utils`) and accessing them from the binary crate's entry point (`main.rs`).

## Project Structure

*   **`src/main.rs`**: The binary crate entry point. It declares the `all_utils` module and calls functions from its submodules.
*   **`src/all_utils/mod.rs`**: The module entry point that declares and exports public submodules.
*   **`src/all_utils/`**: Directory containing implementation modules:
    *   `_1_data_types`: Basic data types demonstration.
    *   `_2_control_statements`: Control flow examples.
    *   `_3_struct`: Structs and object-oriented patterns.
    *   `_4_serde_json`: JSON serialization/deserialization.
    *   `_5_function_ownership`: Functions, ownership, and borrowing.
    *   `_6_static`: Static variables and concurrency (Mutex/RwLock).

## Code Organization

### Module Declaration
In `src/all_utils/mod.rs`, submodules are made public:
```rust
pub mod _1_data_types;
pub mod _2_control_statements;
// ...
```

### Module Usage
In `src/main.rs`, the functionality is accessed via the module path:
```rust
mod all_utils;

fn main() {
    all_utils::_1_data_types::main();
    all_utils::_2_control_statements::main();
    // ...
}
```

## Usage

To run the consolidated examples:

```bash
cargo run
```
