use axum::{routing::{get, post}, Router};
use tower_http::trace::TraceLayer;
use tokio::net::TcpListener;
use std::sync::Arc;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub struct AppState {
   pub  db_pool: PgPool
}

pub async fn run(listener: TcpListener, db_pool: PgPool) {

   let app_state = Arc::new(AppState {db_pool});
    // Define route
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    //Run server
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to start the server");
}