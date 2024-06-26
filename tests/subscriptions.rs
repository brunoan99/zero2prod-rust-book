mod common;
use common::spawn_app;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  let body = "name=bruno&email=nomequenaotemainda%40gmail.com";
  let response = client
    .post(&format!("{}/subscriptions", &app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription");

  assert_eq!(saved.email, "nomequenaotemainda@gmail.com");
  assert_eq!(saved.name, "bruno");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email"),
  ];

  for (invalid_body, error_message) in test_cases {
    // Act
    let response = client
      .post(&format!("{}/subscriptions", &app.address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalid_body)
      .send()
      .await
      .expect("Failed to execute request.");
    // Assert
    assert_eq!(
      400,
      response.status().as_u16(),
      // Additional customised error message on test failure
      "The API did not fail with 400 Bad Request when the payload was {}.",
      error_message
    );
  }
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
    ("name=Ursula&email=", "empty email"),
    ("name=Ursula&email=definitely-not-an-email", "invalid email"),
  ];

  for (body, description) in test_cases {
    // Act
    let response = client
      .post(&format!("{}/subscriptions", &app.address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
      .expect("Failed to execute request.");
    // Assert
    assert_eq!(
      400,
      response.status().as_u16(),
      "The API did not return a 200 OK when the payload was {}.",
      description
    );
  }
}
