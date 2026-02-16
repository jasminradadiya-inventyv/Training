# Rust HTTP Server with Axum

This project demonstrates how to build a **RESTful API** using the **Axum** web framework and **Tokio** runtime. It implements a simple CRUD (Create, Read, Update, Delete) system for managing Employee records, with data persistence in a JSON file.

## Overview

The application runs an HTTP server that exposes endpoints to manage `Employee` data. It uses:
*   **Axum**: For routing and handling HTTP requests.
*   **Tokio**: As the asynchronous runtime.
*   **Serde**: For serializing/deserializing JSON data.
*   **RwLock**: For thread-safe state management across concurrent requests.

## Project Structure

*   `src/main.rs`: Entry point. Sets up the TCP listener, shared state, and routes.
*   `src/routes.rs`: Defines the API routes (`/employees`, `/employees/{id}`).
*   `src/api.rs`: Implementations of the HTTP request handlers (GET, POST, PUT, DELETE).
*   `src/model.rs`: Defines the `Employee` struct.
*   `src/handler.rs`: Manages reading from and writing to `employees.json`.

## Key Concepts

### Shared State in Async Rust
The application shares the list of employees across all incoming requests using `Arc<RwLock<T>>`:
```rust
type SharedState = Arc<RwLock<Vec<Employee>>>;
```
*   `Arc`: Allows multiple owners (handlers) to access the state.
*   `RwLock`: Ensures exclusive access for writes and shared access for reads.

### JSON Persistence
Data is loaded from `employees.json` on startup and saved back to the file whenever a modification (Add, Update, Delete) occurs.

### API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/employees` | Retrieve all employees. |
| `POST` | `/employees` | Add a new employee. |
| `GET` | `/employees/{id}` | Retrieve a specific employee by ID. |
| `PUT` | `/employees/{id}` | Update an existing employee. |
| `DELETE` | `/employees/{id}` | Remove an employee. |

## Usage

To start the server:

```bash
cargo run
```

The server will start at `http://127.0.0.1:4500`.

### Testing with cURL

**Add an Employee:**
```bash
curl -X POST http://127.0.0.1:4500/employees \
   -H "Content-Type: application/json" \
   -d '{"name": "Alice", "email": "alice@example.com", "mobile": "1234567890"}'
```

**Get All Employees:**
```bash
curl http://127.0.0.1:4500/employees
```
