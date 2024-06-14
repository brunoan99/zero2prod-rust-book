use newsletter::configuration::{get_configuration, DatabaseSettings};
use newsletter::startup::create_server;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
  let default_filter_level = "info";
  let subscriber_name = "test";

  if std::env::var("TEST_LOG").is_ok() {
    let subscriber =
      get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
    init_subscriber(subscriber);
  } else {
    let subscriber =
      get_subscriber(subscriber_name, default_filter_level, std::io::sink);
    init_subscriber(subscriber);
  }
});

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
  Lazy::force(&TRACING);

  let listener =
    TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let mut config = get_configuration().expect("Failed to read configuration.");
  config.database.database_name = Uuid::new_v4().to_string();

  let db_pool = configure_database(&config.database).await;

  let server =
    create_server(listener, db_pool.clone()).expect("Failed to create Server");
  let _ = tokio::spawn(server);

  TestApp { address, db_pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
  let mut connection = PgConnection::connect_with(&config.without_db())
    .await
    .expect("Failed to connect to Postgres");
  connection
    .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
    .await
    .expect("Failed to create database.");

  let connection_pool = PgPool::connect_with(config.with_db())
    .await
    .expect("Failed to connect to Postgres after creating db");
  sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database");

  connection_pool
}
