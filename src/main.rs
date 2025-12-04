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
        .route("/categories", get(handlers::category_handler::get_all_category))
        .route("/categories/:id", get(handlers::category_handler::get_category_by_id))
        .route("/categories", post(handlers::category_handler::create_category))
        .route("/categories/:id", put(handlers::category_handler::update_category))
        .route("/categories/:id", delete(handlers::category_handler::delete_category))
        .route("/locations", get(handlers::location_handler::get_all_locations))
        .route("/locations/:id", get(handlers::location_handler::get_location_by_id))
        .route("/locations", post(handlers::location_handler::create_location))
        .route("/locations/:id", put(handlers::location_handler::update_location))
        .route("/locations/:id", delete(handlers::location_handler::delete_location))
        .with_state(pool);

    let addr: SocketAddr = "127.0.0.1:3001".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}
