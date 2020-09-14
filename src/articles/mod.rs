pub mod assets;
pub mod service;
pub mod model;
pub mod views;
pub mod handlers;

use std::env;

use actix_web::{web};
use actix_web::FromRequest;

use crate::articles::model::EditableArticle;
use crate::articles::handlers::{form_error_handler};
use crate::articles::views::{create_article_view, render_articles_view, render_new_article_view, render_article_view};
use crate::articles::assets::{asset_file};

pub fn get_data_directory() -> String
{
  match env::var("CARGO_MANIFEST_DIR") {
    Ok(uri) => format!("{}/{}", uri, "src/articles/data"),
    Err(_) => "/app/data".to_owned(),
  }
}

/// Configure the routes for the articles.
/// 
/// *Note: Form max size is 4Kb*
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      web::scope("")
        .app_data(web::Form::<EditableArticle>::configure(|cfg| { 
          cfg
            .limit(4096)
            .error_handler(form_error_handler)
        }))
        .route("", web::get().to(render_articles_view))
        .route("/assets/{filename:.*}",  web::get().to(asset_file))
        .route("/new", web::get().to(render_new_article_view))
        .route("/new", web::post().to(create_article_view))
        .route("/{article_id}", web::get().to(render_article_view))
    );
}
