pub mod service;
pub mod model;
pub mod views;
pub mod handlers;

use actix_web::{web};
use actix_web::FromRequest;

use crate::articles::model::EditableArticle;
use crate::articles::service::{create_article};
use crate::articles::handlers::{form_error_handler};
use crate::articles::views::{render_articles_view, render_new_article_view, render_article_view};

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("")
    .app_data(web::Form::<EditableArticle>::configure(|cfg| { 
      cfg
        .limit(4096)
        .error_handler(form_error_handler)
    }))
    .route("", web::get().to(render_articles_view))
    .route("/new", web::get().to(render_new_article_view))
    .route("/new", web::post().to(create_article))
    .route("/{article_id}", web::get().to(render_article_view))
  );
}
