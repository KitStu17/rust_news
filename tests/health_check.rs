#[tokio::test]
async fn heath_check_test() {
    // 준비
    spawn_app();

    // reqwest를 사용하여 애플리케이션에게 http 요청을 수행
    let client = reqwest::Client::new();

    // 조작
    let response = client
     .get("http://localhost:8080/health_check")
     .send()
     .await
     .expect("Failed to send request");

    // 결과 확인
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// 백그라운드로 애플리케이션 구동
fn spawn_app(){
    let server = rust_news::run().expect("Failed to spawn app.");
    let _ = tokio::spawn(server);
}