use newsletter::configuration::get_configuration;
use newsletter::startup::create_server;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let subscriber = get_subscriber("newsletter", "info", std::io::stdout);
  init_subscriber(subscriber);

  let configuration =
    get_configuration().expect("Failed to read configuration.");
  let connection_pool = PgPool::connect(
    &configuration.database.connection_string().expose_secret(),
  )
  .await
  .expect("Failed to connect to Postgres.");

  println!("Connected to Database");

  let address = format!("127.0.0.1:{}", configuration.application_port);
  let listener = TcpListener::bind(address).expect("Failed to bind the Port");
  let port = listener.local_addr().unwrap().port();

  println!("Server will run at port:{:?}", port);

  create_server(listener, connection_pool)?.await
}
