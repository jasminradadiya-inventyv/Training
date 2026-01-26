# Rust Serde JSON Example

This project demonstrates how to use the `serde` framework with the `serde_json` crate in Rust to serialize and deserialize data structures to and from JSON format.

## Overview

The application defines nested data structures (`Project` and `Technology`) and performs the following operations:
1.  **Serialization**: Converts a Rust struct instance into a JSON string.
2.  **Deserialization**: Parses a JSON string back into a Rust struct.
3.  **Raw String Handling**: Demonstrates parsing a raw string literal directly into a struct.

## Dependencies

The project uses the following dependencies in `Cargo.toml`:
*   `serde`: For the serialization framework (with `derive` feature enabled).
*   `serde_json`: For JSON-specific implementation.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.128"
```

## Data Structures

Two main structs are used:

*   **Technology**: Contains `frontend` and `backend` fields.
*   **Project**: Contains details like `name`, `tech` (nested Technology), `category`, `no_of_emp`, `no_of_user`, `is_completed`, and `organization`.

Both structs derive `Serialize`, `Deserialize`, and `Debug` traits to facilitate the JSON conversion and printing.

## Usage

To run the project, ensure you have Rust installed and run:

```bash
cargo run
```

## Expected Output

The program will output:
1.  The serialized JSON string derived from a struct.
2.  The deserialized struct printed using the `Debug` formatter (`{:#?}`).
3.  Another deserialized struct derived from a raw JSON string literal.

```text
json_string_ser : {"name":"WebRTC","tech":{"frontend":"Angular","backend":"Rust"},"category":"Communication","no_of_emp":50,"no_of_user":800,"is_completed":false,"organization":"inventyv"}

json_from_str_deser : Project {
    name: "WebRTC",
    tech: Technology {
        frontend: "Angular",
        backend: "Rust",
    },
    category: "Communication",
    no_of_emp: 50,
    no_of_user: 800,
    is_completed: false,
    organization: "inventyv",
}

json_from_raw_string : Project {
    name: "WebRTC",
    tech: Technology {
        frontend: "Angular",
        backend: "Rust",
    },
    category: "Communication",
    no_of_emp: 50,
    no_of_user: 800,
    is_completed: false,
    organization: "inventyv",
}
```
