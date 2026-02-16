use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json
};
use uuid::Uuid;
use std::thread;
use crate::{model::Employee, handler::save_employees, SharedState};

/// GET all employees
pub async fn get_employees(State(state): State<SharedState>) -> Json<Vec<Employee>> {
    let thread_id = thread::current().id();
    println!("get all employees from thread {:?}",thread_id);
    let employees = state.read().await;   
    Json(employees.clone())
}

/// GET employee by id
pub async fn get_employee(Path(id): Path<String>,State(state): State<SharedState>) -> Result<Json<Employee>, StatusCode> {
    let thread_id = thread::current().id();
    println!("get single employee from thread {:?}",thread_id);
    let employees = state.read().await;

    employees.iter()
        .find(|e| e.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// ADD employee
pub async fn add_employee(State(state): State<SharedState>,Json(mut employee): Json<Employee>) -> StatusCode {
    let thread_id = thread::current().id();
    println!("add employee from thread {:?}",thread_id);
    
    employee.id = Uuid::new_v4().to_string();

    let mut employees = state.write().await;
    employees.push(employee);

    save_employees(&employees);

    StatusCode::CREATED
}

/// UPDATE employee
pub async fn update_employee(Path(id): Path<String>,State(state): State<SharedState>,Json(updated_employee): Json<Employee>) -> StatusCode {
    let thread_id = thread::current().id();
    println!("update an employee from thread {:?}",thread_id);

    let mut employees = state.write().await;

    if let Some(employee) = employees.iter_mut().find(|e| e.id == id) {
        employee.name = updated_employee.name;
        employee.email = updated_employee.email;
        employee.mobile = updated_employee.mobile;

        save_employees(&employees);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

/// DELETE employee
pub async fn delete_employee(Path(id): Path<String>,State(state): State<SharedState>) -> StatusCode {
    let thread_id = thread::current().id();
    println!("delete an employee from thread {:?}",thread_id);

    let mut employees = state.write().await;

    if employees.iter().any(|e| e.id == id) {
        employees.retain(|e| e.id != id);
        save_employees(&employees);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
