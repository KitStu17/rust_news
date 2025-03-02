use std::net::TcpListener;
use rust_news::startup::run;
use rust_news::configuration::get_configuration;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 구성 읽기
    let config = get_configuration().expect("Failed to get configuration");
    let connection_pool = PgPool::connect(
        &config.database.connection_string()
    ).await
    .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener,connection_pool)?.await
}