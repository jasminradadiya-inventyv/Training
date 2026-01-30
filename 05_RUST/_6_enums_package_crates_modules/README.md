# Rust Enums, Modules, and Concurrency Primitives

This project demonstrates the organization of Rust code into modules and the use of **Enums** alongside concurrency primitives: **Mutex** and **RwLock**.

## Overview

The application simulates a simple request handling system (GET, POST, DELETE) where global counters track the number of requests. It compares two approaches for managing thread-safe global state:
1.  **Mutex (Mutual Exclusion)**: Ensures only one thread can access the data at a time.
2.  **RwLock (Read-Write Lock)**: Allows multiple readers or one writer at a time.

## Project Structure

The code is organized into modules:
*   `src/main.rs`: The entry point that calls functions from the `mutex` and `rwlock` modules.
*   `src/mutex.rs`: Implements request handling using `std::sync::Mutex`.
*   `src/rwlock.rs`: Implements request handling using `std::sync::RwLock`.

## Key Concepts

### Enums
A `Request` enum is defined in both modules to represent different types of operations:
```rust
enum Request {
    Get { endpoint: String },
    Post { endpoint: String, playload_size: u32 },
    Delete(u32),
}
```

### Global State Management
The project uses `static` variables wrapped in `Mutex` or `RwLock` to maintain global state.

**Mutex approach (`src/mutex.rs`):**
```rust
static TOTAL_REQUESTS: Mutex<usize> = Mutex::new(0);
// ...
*TOTAL_REQUESTS.lock().unwrap() += 1;
```

**RwLock approach (`src/rwlock.rs`):**
```rust
static TOTAL_REQUESTS: RwLock<usize> = RwLock::new(0);
// ...
*TOTAL_REQUESTS.write().unwrap() += 1; // For writing
// ...
*GET_REQUESTS.read().unwrap() // For reading
```

## Usage

To run the project:

```bash
cargo run
```

## Expected Output

The program will execute the Mutex example followed by the RwLock example, printing the results of the simulated requests and the final counter values.

```text
Using Mutex---------
GET endpoint : /images 
=> Total count of GET requst is 1.
POST endpoint : /register playload_size : 50 
=> Total count of POST requst is 1.
DELETE id : 44 
=> Total count of DELETE requst is 1.
The total number of requests are : 3

Using RwLock---------
GET endpoint : /images 
=> Total count of GET request is 1.
POST endpoint : /register playload_size : 52 
=> Total count of POST request is 1.
DELETE id : 44 
=> Total count of DELETE request is 1.
The total number of requests are : 3
```
