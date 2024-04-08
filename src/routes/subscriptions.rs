use axum::http::StatusCode;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use axum::{extract::State, response::IntoResponse, Form};
use serde::Deserialize;

use crate::{domain::NewSubscriber, startup::AppState};


#[derive(Deserialize)]
pub struct  FormData {
    pub email: String,
    pub name: String
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

    let new_subscriber = match form_data.try_into() {
        Ok(form_data) => form_data,
        Err(_) => return StatusCode::BAD_REQUEST
    };

   
     match insert_subscriber(connection, &new_subscriber) .await
    {
        Ok(_) =>  StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR

    }

}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, connection)
)]
pub async fn insert_subscriber(connection: Arc<AppState>, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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