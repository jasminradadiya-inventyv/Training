# Rust Control Flow Statements

## Description
This project demonstrates the usage of various control flow statements in Rust, including loops and conditional branching. It showcases how to control the execution flow using `for`, `while`, `loop`, `if-else`, and `match`.

## Concepts Covered
- **For Loop**: Iterating over a range of numbers.
- **While Loop**: Executing code based on a condition.
- **Loop**: Using an infinite loop with explicit break conditions.
- **If-Else**: Conditional logic execution.
- **Match Statement**: Pattern matching for control flow.

## Code Walkthrough

### 1. For Loop Implementation
Iterates from a start value (`st`) to an end value (`ed`) inclusive.
- Checks if a number is even or odd using `if i % 2 == 0`.
- Prints the result accordingly.

### 2. While Loop Implementation
Runs while `st < ed`.
- Uses a `match` statement to handle specific values:
    - Prints a special message for `5`.
    - Breaks the loop when `st` is `6`.
    - Prints the value for other cases.
- Increments `st` in each iteration.

### 3. Loop (Infinite Loop) Implementation
Demonstrates a `loop` block.
- Prints the current value of `st`.
- Checks if `st` is `3` to print a specific message.
- Uses `match` to decide whether to `break` (at `8`) or `continue`.
