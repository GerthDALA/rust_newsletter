use axum::http::StatusCode;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use axum::{extract::State, response::IntoResponse, Form};
use serde::Deserialize;

use crate::startup::AppState;


#[derive(Deserialize)]
pub struct  FormData {
    email: String,
    name: String
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form_data, connection),
    fields(
        subscriber_email = %form_data.email,
        subscriber_name = %form_data.name
    )
)]
pub async fn subscribe(State(connection): State<Arc<AppState>>, Form(form_data): Form<FormData>) -> impl IntoResponse {
   
     match insert_subscriber(connection, &form_data) .await
    {
        Ok(_) =>  StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR

    }

}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form_data, connection)
)]
pub async fn insert_subscriber(connection: Arc<AppState>, form_data: &FormData) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
    .execute(&connection.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())

}