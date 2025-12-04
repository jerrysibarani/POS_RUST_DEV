use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use sqlx::PgPool;

use crate::models::location::{Location, CreateLocation, UpdateLocation};

pub async fn get_all_locations(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Location>>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Location>("SELECT * FROM locations ORDER BY location_id")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(locations) => Ok(Json(locations)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn get_location_by_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<Location>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Location>(
        "SELECT * FROM locations WHERE location_id = $1",
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(location) => Ok(Json(location)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn create_location(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLocation>,
) -> Result<Json<Location>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Location>(
        "INSERT INTO locations (location_name, address)
         VALUES ($1, $2)
         RETURNING *",
    )
    .bind(payload.location_name)
    .bind(payload.address)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(new_loc) => Ok(Json(new_loc)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn update_location(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateLocation>,
) -> Result<Json<Location>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Location>(
        "UPDATE locations
         SET location_name = COALESCE($1, location_name),
             address       = COALESCE($2, address)
         WHERE location_id = $3
         RETURNING *",
    )
    .bind(payload.location_name)
    .bind(payload.address)
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(updated) => Ok(Json(updated)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn delete_location(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let result = sqlx::query(
        "DELETE FROM locations WHERE location_id = $1",
    )
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({ "message": "Location deleted" }))),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}
