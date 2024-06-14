use actix_web::{dev, web, App, HttpServer};
use routes::{health_check, subscriptions};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes;

pub fn create_server(
  listener: TcpListener,
  db_pool: Pool<Postgres>,
) -> Result<dev::Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
    App::new()
      .wrap(TracingLogger::default())
      .route("/health_check", web::get().to(health_check::health_check))
      .route("/subscriptions", web::post().to(subscriptions::subscribe))
      .app_data(db_pool.clone())
  })
  .listen(listener)?
  .run();

  Ok(server)
}
