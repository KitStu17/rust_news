use std::net::TcpListener;
use rust_news::startup::run;
use rust_news::configuration::get_configuration;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 모든 log 이벤트를 구독자에게 리다이렉트함 
    LogTracer::init().expect("Failed to set logger");
    // init()는 set_logger()를 호출
    // RUST_LOG 환경변수가 설정되어 있지 않으면
    // info 및 그 이상의 모든 레벨의 span을 출력
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 포맷이 적용된 span들을 stdout으로 출력
    let formatting_layer = BunyanFormattingLayer::new("rust_news".into(), std::io::stdout);
    let subscriber = Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);
    // 에플리케이션이 span을 처리할 수 있도록 subscriber 지정 
    set_global_default(subscriber).expect("Failed to set subscriber");
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