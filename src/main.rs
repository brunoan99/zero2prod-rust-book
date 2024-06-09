use newsletter::create_server;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind the Port");
  let port = listener.local_addr().unwrap().port();
  println!("Server running at :{:?}", port);
  create_server(listener)?.await
}
