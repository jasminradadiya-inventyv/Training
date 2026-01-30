# Rust Functions, Ownership, Borrowing, and References

This project explores core Rust concepts: **Functions**, **Ownership**, **Borrowing**, and **References**, using a `Project` management system as an example.

## Overview

The application demonstrates how to define and use structs with methods that take different types of `self` arguments to handle ownership and borrowing rules effectively. It also shows how to declare and use mutable references.

## Key Concepts

### Structs and Methods
*   **Struct Definitions**: `Project` and `Technology` structs store related data.
*   **implementation Blocks**: `impl Project` defines methods associated with the struct.
*   **Getters and Setters**: Custom methods to read (get) and modify (set) private fields.
    *   `set_name(&mut self, ...)`: Takes a mutable reference to modify the instance.
    *   `get_name(&self) -> ...`: Takes an immutable reference to read data without taking ownership.

### Ownership and Borrowing
*   **Immutable Borrowing (`&self`)**: Methods that read data borrow the instance immutably.
*   **Mutable Borrowing (`&mut self`)**: Methods that modify data borrow the instance mutably.
*   **References (`&`, `&mut`)**: The `main` function creates a mutable reference `p2` to `p1`, allowing modification of `p1` through `p2`.

### Memory Addresses
The code demonstrates how to inspect memory addresses using the `{:p}` formatter, showing the relationship between a variable and its references.

```rust
println!("{:p}", p2);  // Address the reference points to
println!("{:p}", &p2); // Address of the reference variable itself
```

## Usage

To run the project:

```bash
cargo run
```

## Expected Output

The program modifies a project instance via a mutable reference and prints the updated details alongside memory addresses.

```text
1)Address : 0x...,
 Name of project : New Project,
 ...
 Organization name : inventyv

1)Address : 0x...,
 Name of project : New Project,
 ...
 Backend Tech : NODE.JS,
 ...

0x... (Address of p1 via p2)
0x... (Address of p2)
0x... (Address of p1)
```
