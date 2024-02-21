use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    // spawn_app().await;// .expect("Failed to spawn our app.")
    let client = reqwest::Client::new();
    // let response = client.get("http://127.0.0.1:8000/health_check").send().await.expect("Failed to execute request.");

    let response = client.get(&format!("{}/health_check", &address)).send().await.expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String { //-> Result<(), std::io::Error>
    // zero2prod::run().await
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).await.expect("Failed to bind the address.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}