use std::net::TcpListener;

#[tokio::test]
async fn heath_check_test() {
    // 준비
    let addr = spawn_app();

    // reqwest를 사용하여 애플리케이션에게 http 요청을 수행
    let client = reqwest::Client::new();

    // 조작
    let response = client
     .get(&format!("{}/health_check", &addr))
     .send()
     .await
     .expect("Failed to send request");

    // 결과 확인
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// 백그라운드로 애플리케이션 구동
fn spawn_app()-> String{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = rust_news::run(listener).expect("Failed to spawn app.");
    let _ = tokio::spawn(server);
    format!("http://localhost:{}", port)
}