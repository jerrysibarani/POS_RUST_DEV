mod db;
mod models;
mod handlers;

use axum::{routing::{get, post, put, delete}, Router};
use db::connect_db;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = connect_db().await;

    let app = Router::new()
        .route("/units", get(handlers::unit_handler::get_all_units))
        .route("/units/:id", get(handlers::unit_handler::get_unit_by_id))
        .route("/units", post(handlers::unit_handler::create_unit))
        .route("/units/:id", put(handlers::unit_handler::update_unit))
        .route("/units/:id", delete(handlers::unit_handler::delete_unit))
        .with_state(pool);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}
