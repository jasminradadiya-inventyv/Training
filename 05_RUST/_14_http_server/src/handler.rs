use std::fs;
use crate::model::Employee;

/// Load employees from file
pub fn load_employees() -> Vec<Employee> {
    fs::read_to_string("employees.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
        .unwrap_or_else(|_| vec![])
}

/// Save employees to file
pub fn save_employees(employees: &[Employee]) {
    fs::write(
        "employees.json",
        serde_json::to_string_pretty(employees).unwrap()
    ).unwrap();
}
