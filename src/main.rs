use rust_newsletter::{configuration::get_configuration, telemetry::{get_subscriber, init_subscriber}};
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("rust_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber); 
    
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    
    let addr = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    rust_newsletter::startup::run(listener, connection_pool).await;
    Ok(())
}
 