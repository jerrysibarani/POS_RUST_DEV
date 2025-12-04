use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Location {
    pub location_id: i32,
    pub location_name: String,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocation {
    pub location_name: String,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLocation {
    pub location_name: Option<String>,
    pub address: Option<String>,
}
