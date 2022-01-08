//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // we need to bring in `reqwest`
    // to perform HTTP requests against our app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // but we have no use for it so use non binding let
    let _ = tokio::spawn(server);
}
