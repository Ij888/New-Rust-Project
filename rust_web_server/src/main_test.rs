use reqwest::Client;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_server_response() {
    // Start the server in a separate task
    let server_task = tokio::spawn(async {
        main().await;
    });

    // Wait for the server to start
    sleep(Duration::from_secs(1)).await;

    // Send a GET request to the server
    let client = Client::new();
    let response = client.get("http://localhost:8080").send().await.unwrap();

    // Assert the response status code and body
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let body = response.text().await.unwrap();
    assert_eq!(body, "Server is running...");

    // Stop the server
    server_task.abort();
}