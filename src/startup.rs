use actix_web::{dev, middleware::Logger, web, App, HttpServer};
use routes::{health_check, subscriptions};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes;

pub fn create_server(
  listener: TcpListener,
  db_pool: PgPool,
) -> Result<dev::Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .route("/health_check", web::get().to(health_check::health_check))
      .route("/subscriptions", web::post().to(subscriptions::subscribe))
      .app_data(db_pool.clone())
  })
  .listen(listener)?
  .run();

  Ok(server)
}
