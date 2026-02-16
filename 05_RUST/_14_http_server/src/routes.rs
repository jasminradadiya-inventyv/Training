use axum::{ Router, routing::get };

use crate::{
    api::{ get_employees, get_employee, add_employee, update_employee, delete_employee },
    SharedState,
};

pub fn create_routes(state: SharedState) -> Router {
    Router::new()
        .route("/employees", get(get_employees).post(add_employee))
        .route("/employees/{id}", get(get_employee).put(update_employee).delete(delete_employee))
        .with_state(state)
}
