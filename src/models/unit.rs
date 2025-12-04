use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Unit {
    pub ID: i32,
    pub UnitCode: String,
    pub UnitName: String,
    pub UnitDesc: Option<String>,
    pub IsActive: Option<bool>,
    pub CreatedBy: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub LastUpdatedBy: Option<String>,
    pub LastUpdatedAt: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUnitRequest {
    pub UnitCode: String,
    pub UnitName: String,
    pub UnitDesc: Option<String>,
    pub IsActive: Option<bool>,
    pub CreatedBy: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUnitRequest {
    pub UnitName: Option<String>,
    pub UnitDesc: Option<String>,
    pub IsActive: Option<bool>,
    pub LastUpdatedBy: Option<String>,
}
