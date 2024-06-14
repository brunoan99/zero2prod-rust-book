use newsletter::configuration::get_configuration;
use newsletter::startup::create_server;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let subscriber = get_subscriber("newsletter", "info", std::io::stdout);
  init_subscriber(subscriber);

  let configuration =
    get_configuration().expect("Failed to read configuration.");
  let connection_pool =
    PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

  println!("Connected to Database");

  let address = format!(
    "{}:{}",
    configuration.application.host, configuration.application.port
  );
  let listener = TcpListener::bind(address).expect("Failed to bind the Port");
  let port = listener.local_addr().unwrap().port();

  println!("Server will run at port:{:?}", port);

  create_server(listener, connection_pool)?.await
}
