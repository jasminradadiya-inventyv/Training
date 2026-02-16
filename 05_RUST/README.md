# Rust Programming Training

This folder contains a collection of Rust projects and exercises, each focusing on specific language concepts and features.

## Directory Structure

```
05_RUST/
├── _2_task_control_statements/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── _3_structs_and_related_data/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── _4_serde_json/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── _5_functions_ownership_borrowing_and_references/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── _6_static_variables/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── main.rs
│       ├── mutex.rs
│       └── rwlock.rs
├── _7_packages_crates_modules/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── main.rs
│       └── all_utils/
│           ├── mod.rs
│           ├── domain_layer/
│           │   ├── mod.rs
│           │   └── ...
│           └── infrastructure_layer/
│               ├── mod.rs
│               └── ...
├── _8_hashmap_hashset/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── _11_generic_trait_assignment/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── _13_multithread/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
└── _14_http_server/
    ├── Cargo.toml
    ├── README.md
    ├── employees.json
    └── src/
        ├── api.rs
        ├── handler.rs
        ├── main.rs
        ├── model.rs
        └── routes.rs
```

## Project List

### Basics & Control Flow
*   **_2_task_control_statements**: Covers fundamental control flow constructs like `if-else`, `match`, `for`, `while`, and `loop`.
*   **_5_functions_ownership_borrowing_and_references**: Explores Rust's core memory safety features: ownership, borrowing, and references.

### Data Structures & Types
*   **_3_structs_and_related_data**: Introduces `structs` for data modeling, along with methods and associated functions.
*   **_4_serde_json**: Demonstrates JSON serialization and deserialization using the `serde` crate.
*   **_6_static_variables**: Shows how to use global `static` variables and thread-safe primitives like `Mutex` and `RwLock`.
*   **_8_hashmap_hashset**: Covers standard collection types `HashMap` and `HashSet` for storing key-value pairs and unique items.

### Module System
*   **_7_packages_crates_modules**: Explains how to organize code into packages, crates, and reusable modules.

### Advanced Concepts
*   **_11_generic_trait_assignment**: Implements a generic inventory system using `Generics` and `Traits` (including custom error handling).
*   **_13_multithread**: Simulates a concurrent system using multiple threads, shared state (`Arc`, `RwLock`), and atomic operations.

### Web Development
*   **_14_http_server**: A full RESTful API built with the **Axum** framework and **Tokio** runtime, featuring JSON persistence and concurrent request handling.
