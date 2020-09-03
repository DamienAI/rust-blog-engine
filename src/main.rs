use std::env;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};

pub mod application;
pub mod articles;
pub mod database;

static SERVER_BINDING: &str = "0.0.0.0:4000";
static MONGODB_URI: &str = "mongodb://root:tutorial@172.21.0.2:27017";
static MONGODB_DATABASE: &str = "tutorials";

// https://www.youtube.com/watch?v=1NrHkjlWVhM
// https://crates.io/crates/ammonia


// Example blog
// https://www.free-css.com/free-css-templates/page255/devblog-v1.1

/// Entry point of the rest-api server
///
/// # Panics
/// 
/// Will panic if it cannot connect to mongo or it cannot spawn the actix server.
///
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let mongo_uri = match env::var("MONGO_URI") {
    Ok(uri) => uri,
    Err(_) => MONGODB_URI.to_owned(),
  };

  let mongo_database = match env::var("MONGO_DATABASE") {
    Ok(db_name) => db_name,
    Err(_) => MONGODB_DATABASE.to_owned(),
  };

  let db = match database::connect(mongo_uri.as_str(), mongo_database.as_str()).await {
    Err(e) => {
      panic!(e);
    },
    Ok(mongo_db) => mongo_db,
  };

  let server = HttpServer::new(move || {
    App::new()
      .data(application::AppData { db: db.clone(), articles: articles::views::get_app_data() })
      .wrap(middleware::Logger::new("%U - %s - %b bytes - %D ms"))
      .wrap(
        Cors::new()
          // .allowed_origin("*")
          .allowed_methods(vec!["GET", "PATCH", "POST", "OPTIONS"])
          .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
          .max_age(500)
          .finish()
      )
      .service(
        web::scope("/articles")
          .configure(articles::configure)
      )
  });

  let result = server.bind(SERVER_BINDING);

  match result {
    Ok(server_instance) => {
      println!("Server listening on {}", SERVER_BINDING);
      server_instance.run().await
    },
    Err(e) => {
      println!("Cannot bind server on {} {}", SERVER_BINDING, e);
      Err(e)
    }
  }
}
