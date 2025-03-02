use std::net::TcpListener;
use sqlx::{PgPool};
use rust_news::startup::run;
use rust_news::configuration::get_configuration;

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
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to database");
    let server = rust_news::startup::run(listener, connection_pool.clone()).expect("Failed to spawn app.");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}