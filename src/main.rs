use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::create_server;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let subscriber = get_subscriber("newsletter", "info", std::io::stdout);
  init_subscriber(subscriber);

  let configuration =
    get_configuration().expect("Failed to read configuration.");
  let connection_pool =
    PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

  let address = format!(
    "{}:{}",
    configuration.application.host, configuration.application.port
  );
  let listener = TcpListener::bind(address).expect("Failed to bind the Port");
  let port = listener.local_addr().unwrap().port();

  println!("Server will run at port:{:?}", port);

  create_server(listener, connection_pool)?.await
}
