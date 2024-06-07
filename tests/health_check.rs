use std::net::TcpListener;

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let server = newsletter::create_server(listener).expect("Failed to create Server");
  let _ = tokio::spawn(server);

  format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
  let address = spawn_app();
  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request.");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
  let address = spawn_app();
  let client = reqwest::Client::new();

  let body = "email=nomequenaotemainda%40gmail.com";
  let response = client
    .post(&format!("{}/subscriptions", &address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
  let address = spawn_app();
  let client = reqwest::Client::new();

  let response = client
    .post(&format!("{}/subscriptions", &address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body("")
    .send()
    .await
    .expect("Failed to execute request.");

  assert_eq!(400, response.status().as_u16());
}
