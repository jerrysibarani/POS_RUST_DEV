use axum::{Json, extract::{Path, State}};
use sqlx::PgPool;

use crate::models::unit::{Unit, CreateUnitRequest, UpdateUnitRequest};
use chrono::Utc;

pub async fn get_all_units(
    State(pool): State<PgPool>
) -> Json<Vec<Unit>> {
    let rows = sqlx::query_as::<_, Unit>(r#"SELECT * FROM "Units" ORDER BY "ID""#)
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}

pub async fn get_unit_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Option<Unit>> {
    let row = sqlx::query_as::<_, Unit>(r#"SELECT * FROM "Units" WHERE "ID" = $1"#)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    Json(row)
}

pub async fn create_unit(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUnitRequest>,
) -> Json<Unit> {
    let created_at = Utc::now().naive_utc();

    let row = sqlx::query_as::<_, Unit>(
        r#"
        INSERT INTO "Units"
        ("UnitCode", "UnitName", "UnitDesc", "IsActive", "CreatedBy", "CreatedAt")
        VALUES ($1,$2,$3,$4,$5,$6)
        RETURNING *
        "#
    )
    .bind(payload.UnitCode)
    .bind(payload.UnitName)
    .bind(payload.UnitDesc)
    .bind(payload.IsActive)
    .bind(payload.CreatedBy)
    .bind(created_at)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(row)
}

pub async fn update_unit(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUnitRequest>,
) -> Json<Option<Unit>> {
    let last_updated_at = Utc::now().naive_utc();

    let row = sqlx::query_as::<_, Unit>(
        r#"
        UPDATE "Units"
        SET 
            "UnitName" = COALESCE($1, "UnitName"),
            "UnitDesc" = COALESCE($2, "UnitDesc"),
            "IsActive" = COALESCE($3, "IsActive"),
            "LastUpdatedBy" = COALESCE($4, "LastUpdatedBy"),
            "LastUpdatedAt" = $5
        WHERE "ID" = $6
        RETURNING *
        "#
    )
    .bind(payload.UnitName)
    .bind(payload.UnitDesc)
    .bind(payload.IsActive)
    .bind(payload.LastUpdatedBy)
    .bind(last_updated_at)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .unwrap();

    Json(row)
}

pub async fn delete_unit(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<bool> {
    let result = sqlx::query(r#"DELETE FROM "Units" WHERE "ID" = $1"#)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Json(result.rows_affected() > 0)
}
