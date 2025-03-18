use std::net::TcpListener;
use rust_news::startup::run;
use rust_news::configuration::get_configuration;
use sqlx::PgPool;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // init()는 set_logger()를 호출
    // RUST_LOG 환경변수가 설정되어 있지 않으면
    // info 및 그 이상의 모든 레벨의 로그를 출력함
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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