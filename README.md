# Training Repository

## Folder Structure

```
Training/
├── 01_Logic Building/
│   └── FlowCharts.md
├── 02_HTML_CSS/
│   ├── index.html
│   ├── style.css
│   └── README.md
├── 03_SQL_Queries/
│   └── Quries_with_Solution.md
├── 04_JavaScript_Tasks/
│   ├── Exercism_Javascript/
│   ├── TASK_1/
│   └── TASK_3/
├── 05_RUST/
│   ├── _2_task_control_statements/
│   ├── _3_structs_and_related_data/
│   ├── _4_serde_json/
│   ├── _5_functions_ownership_borrowing_and_references/
│   ├── _6_static_variables/
│   ├── _7_packages_crates_modules/
│   └── _8_hashmap_hashset/
└── README.md
```

## 01_Logic Building

This folder illustrates how I approach algorithmic thinking before writing actual code.

*   **FlowCharts.md**: I created 12 flowchart diagrams to visualize problem-solving steps. These cover basic logic like finding the maximum of multiple numbers, mathematical series like Factorials and Fibonacci, and generating complex number patterns.

## 02_HTML_CSS

I created a responsive Login and Registration page using pure HTML and CSS.

*   **Tabbed Interface**: Implemented a toggle mechanism to switch between Login and Register forms without page reloading.
*   **Design**: Focused on a clean, split-screen layout with responsive styling for various screen sizes.

## 03_SQL_Queries

I practiced database management using a movie-related dataset (movies, actors, directors, reviewers).

*   **Quries_with_Solution.md**: Contains 50 queries progressing from simple to complex.
    *   **Basics**: Simple selects, filtering by dates, and pattern matching.
    *   **Sub-Queries**: Nested requests to find specific data like actors in a certain movie.
    *   **Joins**: Linking multiple tables to connect directors, actors, and ratings in a single view.

## 04_JavaScript_Tasks

Here I implemented scripts to handle arrays and asynchronous operations.

*   **Exercism_Javascript**: Contains solutions for Exercism JavaScript track exercises.
*   **TASK_1**: I built a script that takes an existing array, modifies it by determining specific elements to move, and creates a new combined array. Finally, I used a Promise to validate if the total sum meets a specific condition.
*   **TASK_3**: This follows a similar logic but introduces JavaScript Symbols. I used a Symbol as a unique key to store and retrieve the array from an object, demonstrating how to handle unique properties safe from name collisions.

## 05_RUST

This directory contains Rust programming tasks focusing on core language concepts.

*   **_2_task_control_statements**:
    *   Demonstrates the usage of control flow statements such as `for`, `while`, `loop`, `if-else`, and `match`.
    *   Includes examples of iterating over ranges, conditional execution, and handling infinite loops with break conditions.

*   **_3_structs_and_related_data**:
    *   Focuses on defining and using **Structs** to model data (e.g., a `Project` entity).
    *   Implements **methods** (getters and setters) for encapsulation.
    *   Demonstrates **associated functions** (static methods) and object instantiation.

*   **_4_serde_json**:
    *   Showcases how to use **Serde** and **serde_json** for serializing and deserializing data.
    *   Includes examples of converting structs to JSON strings, parsing JSON strings back to structs, and handling raw JSON literals.

*   **_5_functions_ownership_borrowing_and_references**:
    *   Covers fundamental memory management concepts: **Ownership**, **Borrowing** (mutable and immutable), and **References**.
    *   Demonstrates struct methods with different `self` receivers (`&self`, `&mut self`).
    *   Includes examples of modifying data via mutable references and visualizing memory addresses.

*   **_6_static_variables**:
    *   Explores **static variables** and **concurrency primitives** in Rust: `Mutex` and `RwLock`.
    *   Demonstrates how to safely manage global state across threads/calls using `std::sync` primitives.
    *   Uses **Enums** to model different request types in a simulated system.

*   **_7_packages_crates_modules**:
    *   Demonstrates Rust's module system: **Packages**, **Crates**, and **Modules**.
    *   Shows how to organize code by splitting it into multiple files and directories (`src/all_utils`).
    *   Consolidates previous concepts (structs, serde, functions) into a modular library structure.

*   **_8_hashmap_hashset**:
    *   Demonstrates the usage of Rust's standard collection types: **HashMap** and **HashSet**.
    *   Showcases basic operations: keys, values, insertion, removal, and filtering (`retain`).
    *   Uses custom structs with derived `Eq` and `Hash` traits as values/keys.