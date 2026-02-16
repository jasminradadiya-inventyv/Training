use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Employee {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
}
