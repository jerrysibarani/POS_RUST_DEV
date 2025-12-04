use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use sqlx::PgPool;

use crate::models::category::{Category, CreateCategory, UpdateCategory};

pub async fn get_all_category(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Category>>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY category_id")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(categories) => Ok(Json(categories)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn get_category_by_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<Category>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE category_id = $1",
    )
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(cat) => Ok(Json(cat)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn create_category(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCategory>,
) -> Result<Json<Category>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (category_name, description)
         VALUES ($1, $2)
         RETURNING *",
    )
        .bind(payload.category_name)
        .bind(payload.description)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(new_cat) => Ok(Json(new_cat)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn update_category(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateCategory>,
) -> Result<Json<Category>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, Category>(
        "UPDATE categories
         SET category_name = COALESCE($1, category_name),
             description   = COALESCE($2, description)
         WHERE category_id = $3
         RETURNING *",
    )
        .bind(payload.category_name)
        .bind(payload.description)
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(updated) => Ok(Json(updated)),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn delete_category(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let result = sqlx::query(
        "DELETE FROM categories WHERE category_id = $1",
    )
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(json!({ "message": "Category deleted" }))),
        Err(e) => Err(Json(json!({ "error": e.to_string() }))),
    }
}
