use sqlx::{Pool, Postgres};
use std::env;

pub async fn connect_db() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

    Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}
