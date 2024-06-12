use env_logger::Env;
use newsletter::configuration::get_configuration;
use newsletter::startup::create_server;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let configuration = get_configuration().expect("Failed to read configuration.");
  let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");

  println!("Connected to Database");

  let address = format!("127.0.0.1:{}", configuration.application_port);
  let listener = TcpListener::bind(address).expect("Failed to bind the Port");
  let port = listener.local_addr().unwrap().port();

  println!("Server will run at port:{:?}", port);

  create_server(listener, connection_pool)?.await
}
