# Rust Generics, Traits, and Error Handling

This project builds a generic **Inventory System** to demonstrate the power of **Generics** and **Traits** in Rust. It also includes custom error handling for robust application logic.

## Overview

The application defines a generic `Inventory<T>` that can store any type of item, provided the item implements specific traits (like `Clone` and `DisplayItem`). It showcases:
*   **Generics**: Defining structs and implementation blocks that work with any appropriate type.
*   **Traits**: Defining shared behavior (`DisplayItem`) and implementing standard traits (`Display` for errors).
    *   Note: The `Inventory` implementation requires items to implement `Clone` for retrieval (`cloned()`).
*   **Error Handling**: Using a custom `InventoryError` enum to manage failure states.

## Key Concepts

### Generics
The core struct uses a generic type parameter `T`:
```rust
struct Inventory<T> {
    items: HashMap<String, T>,
}
```

### Traits
A custom trait `DisplayItem` enforces that stored items must know how to display themselves:
```rust
trait DisplayItem {
    fn display(&self) -> Result<String, InventoryError>;
}
```
This allow the `Inventory` to call `.display()?` on any item it stores, without knowing the exact type.

### Error Handling
A custom enum `InventoryError` handles specific error cases:
*   `DuplicateId`: Attempting to add an item with an existing ID.
*   `InvalidId`: Using an empty or malformed ID.
*   `ItemNotFound`: Searching for an ID that doesn't exist.

It implements `std::fmt::Display` to provide user-friendly error messages.

## Usage

The `main` function demonstrates creating an `Inventory` of `Product` structs, adding items, and retrieving/displaying them.

To run the project:

```bash
cargo run
```

## Expected Output

The program attempts to fetch a non-existent item (printing the error) and then displays the full inventory.

```text
Item from found is Err(
    ItemNotFound,
)

Id: 1
Product: Laptop
Price: 75000

Id: 2
Product: Phone
Price: 30000
```
