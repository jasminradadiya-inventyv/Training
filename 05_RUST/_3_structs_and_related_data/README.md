# Rust Structs and Related Data

## Description
This project demonstrates the use of **Structs** in Rust to model real-world entities. It covers defining structures, implementing methods (getters and setters), nesting structs, and using associated functions (static methods).

## Concepts Covered
- **Struct Definition**: Defining custom data types (`Project` and `Technology`).
- **Nested Structs**: Using one struct (`Technology`) as a field within another (`Project`).
- **Impl Block**: Implementing functionality for structs.
- **Methods**:
    - **Getters**: Accessing data (`&self`).
    - **Setters**: Modifying data (`&mut self`).
- **Associated Functions**: Functions defined within an `impl` block that do not take `self` as a parameter (similar to static methods).
- **Format Macro**: Using `format!` to create formatted strings.

## Code Walkthrough

### 1. Struct Definitions
- **Technology**: Holds `frontend` and `backend` stack details.
- **Project**: Represents a project with details like name, technology, category, employee count, user count, completion status, and organization.

### 2. Method Implementations
The `impl Project` block defines several methods:
- **Setters**: Methods like `set_name`, `set_frontend`, `set_backend`, etc., take `&mut self` to modify the instance.
- **Getters**: Methods like `get_name`, `get_frontend`, etc., take `&self` to return values. 
    - *Note*: `get_is_completed` returns a descriptive string ("It is completed" or "It is not completed!") instead of a raw boolean.
- **Full Info**: `get_full_project` returns a multi-line string summarizing all project details.

### 3. Associated Function
- `get_full_project_wo_self`: This function allows generating a project summary string without creating a struct instance first. It takes individual parameters and returns the formatted string.

### 4. Main Execution
- Creates an initial `Project` instance (`p1`) and prints details using getters.
- Creates an empty `custom_project` instance.
- Updates `custom_project` using setter methods to populate logical data.
- Prints the updated project details.
- Calls `Project::get_full_project_wo_self` directly to demonstrate invoking associated functions.
