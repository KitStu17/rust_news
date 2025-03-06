use std::net::TcpListener;
use sqlx::{Connection, PgConnection, PgPool, Executor};
use rust_news::startup::run;
use rust_news::configuration::{get_configuration, DatabaseSettings};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn heath_check_test() {
    // 준비
    let app = spawn_app().await;

    // reqwest를 사용하여 애플리케이션에게 http 요청을 수행
    let client = reqwest::Client::new();

    // 조작
    let response = client
     .get(&format!("{}/health_check", &app.address))
     .send()
     .await
     .expect("Failed to send request");

    // 결과 확인
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_form_response_200() {
    // 준비
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // 조작
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // 결과 확인
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to execute query");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

// 현재 "/subscriptions" API는 무조건 200 OK 반환
// 해당 테스트는 failed 처리 되어야함
#[tokio::test]
async fn subscribe_form_response_400() {
    // 준비
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for(invalid_body, error_msg) in test_cases {
        // 조작
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // 결과 확인
        assert_eq!(
            400,
            response.status().as_u16(),
            // 오류 메세지 커스터마이징
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        )
    }
}

// 백그라운드로 애플리케이션 구동
async fn spawn_app()-> TestApp{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // let configuration = get_configuration().expect("Failed to get configuration");
    // 고유한 이름의 새로운 논리 DB 생성
    // let mut configuration = get_configuration().expect("Failed to get configuration");
    // configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database().await;
    
    // let connection_pool = PgPool::connect(
    //     &configuration.database.connection_string()
    // )
    // .await
    // .expect("Failed to connect to database");

    let server = run(listener, connection_pool.clone()).expect("Failed to spawn app.");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database() -> PgPool {
    let mut configuration = get_configuration().expect("Failed to get configuration");
    // DB 생성
    println!("{}\n",&configuration.database.connection_string_without_db());
    let mut connection = PgConnection::connect(&configuration.database.connection_string()).await.expect("Failed to connect to database");
    configuration.database.database_name = Uuid::new_v4().to_string();

    // 데이터베이스가 존재하는지 확인하고, 존재하지 않으면 생성
    let database_exists = connection.fetch_one(format!("SELECT 1 FROM pg_database WHERE datname = '{}'", configuration.database.database_name).as_str()).await.is_ok();
    if !database_exists {
        connection.execute(format!(r#"CREATE DATABASE "{}";"#, configuration.database.database_name).as_str()).await.expect("Failed to create database");
    }

    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to database");

    sqlx::migrate!("./migrations").run(&connection_pool).await.expect("Failed to migrate the database");

    connection_pool
}