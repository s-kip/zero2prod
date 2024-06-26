#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.8/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<(), std::io::Error> {
    zero2prod::run().await
}
