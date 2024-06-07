use actix_web::dev::Server;
use actix_web::web::{get, post};
use actix_web::{App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().finish()
}

async fn subscribe() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn create_server(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/health_check", get().to(health_check))
      .route("/subscriptions", post().to(subscribe))
  })
  .listen(listener)?
  .run();

  Ok(server)
}
