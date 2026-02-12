# Rust Multithreading and Concurrency

This project demonstrates advanced concurrency patterns in Rust using **Threads**, **Shared State** (`Arc`, `RwLock`), and **Atomic Operations**. It simulates a real-time system with multiple producers, consumers, and background maintenance tasks operating on a shared dataset.

## Overview

The application creates a shared vector of `MultiThread` records and spawns multiple threads to perform different operations concurrently:
*   **Producer**: Adds new records.
*   **Printer**: Displays current records.
*   **Cleaners**: Remove old records based on criteria (even/odd IDs).
*   **Counters**: Count specific types of records (even/odd IDs).

## Key Concepts

### Shared Ownership and Mutability
*   **`Arc<T>` (Atomic Reference Counted)**: Allows multiple threads to own the shared data structure.
*   **`RwLock<T>` (Read-Write Lock)**: Allows multiple readers *or* one writer at a time, ensuring thread safety.
    *   `data.read().unwrap()`: Obtains read access (used by Printer, Counters).
    *   `data.write().unwrap()`: Obtains write access (used by Producer, Cleaners).

### Atomic Operations
*   **`AtomicI32`**: Used for generating unique, thread-safe IDs via `fetch_add`.

### Thread Management
*   **`thread::spawn`**: Creates new OS threads.
*   **`thread::sleep`**: Pauses thread execution to simulate work or intervals.

## Architecture

The system consists of the following threads loop:

1.  **Producer**: Adds a new record every 10 seconds.
2.  **Printer**: Prints all current records every 5 seconds.
3.  **Even Cleaner**: Every 20 seconds, removes records with **Even IDs** older than 20 seconds.
4.  **Odd Cleaner**: Every 20 seconds, removes records with **Odd IDs** older than 20 seconds.
5.  **Even/Odd Counters**: Periodically count and print the number of even/odd records.

## Usage

To run the simulation:

```bash
cargo run
```

The program will run for roughly 200 seconds before terminating.

## Expected Output

You will see interleaved logs from different threads, demonstrating the concurrent nature of the application:

```text
[   5s][EVEN COUNT] 0
[   5s][ODD COUNT] 0
[  10s][PRODUCER] Added ID=1 | Total=1
[  20s][EVEN CLEANER] Removed 0
...
```
