# Rust HashMaps and HashSets

This project demonstrates the usage of Rust's standard collection types: **HashMap** and **HashSet**. It focuses on storing, retrieving, modifying, and filtering data using a custom `Custom` struct.

## Overview

The application performs operations on a map of users (keyed by ID) and a set of unique users.
*   **HashMap**: Stores key-value pairs (`u32` -> `Custom` struct).
*   **HashSet**: Stores unique values (`Custom` struct).

## Key Concepts

### HashMap Operations
The `hashmap()` function demonstrates:
*   **Creation & Reservation**: `HashMap::new()` and `try_reserve()` for optimization.
*   **Insertion**: Adding elements with `insert()`.
*   **Iteration**: Looping through key-value pairs.
*   **Filtering**: Using `retain()` to keep elements matching a condition (e.g., name starts with 'J').
*   **Extension**: Merging another map using `extend()`.
*   **Retrieval**: Safe access using `get()`.

### HashSet Operations
The `hashset()` function demonstrates:
*   **Conversion**: creating a set from existing map values to ensure uniqueness.
*   **Removal**: Using `take()` to remove and return a specific item.
*   **Filtering**: Using `retain()` to keep elements matching a condition (e.g., email ends with ".com").

### Custom Structs in Collections
The `Custom` struct derives `PartialEq`, `Eq`, and `Hash` traits, which are required for it to be stored in a `HashSet` or used as a key.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Custom {
    name: String,
    email: String,
}
```

## Usage

To run the examples:

```bash
cargo run
```

## Expected Output

The program prints the state of the map/set after various operations:
1.  All users in the initial map.
2.  Filtered users (Retained starting with 'J').
3.  Extended list with new users.
4.  A specific user fetched by ID.
5.  Unique users in a HashSet.
6.  Remaining users in the set after removal and filtering.
