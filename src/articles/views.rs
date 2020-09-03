use tera::{Tera, Context};
use actix_web::{HttpResponse, Responder, web};
use mongodb::{bson};

use crate::application::AppData;
use crate::articles::service::{find_one_article_by_id, get_articles, insert_article, parse_markdown};
use crate::articles::model::{EditableArticle, RenderableArticle};

/// Structure where we store data related to articles content.
/// These data is provided to the endpoints by the actix server and are stored in thread safe objects.
pub struct ArticlesAppData
{
  /// Provides the templates processor.
  templates: Tera,
}

/// Provides the application data for the articles.
pub fn get_app_data() -> ArticlesAppData {
  let tera = Tera::new(
    concat!(env!("CARGO_MANIFEST_DIR"), "/src/articles/templates/**/*")
  ).unwrap();

  ArticlesAppData{ templates: tera }
}

/// Render the list of articles using the articles from the Mongo database and return the html page.
pub async fn render_articles_view(app_data: web::Data<AppData>) -> impl Responder {
  let articles = match get_articles(&app_data.get_ref().db).await {
    Ok(result) => result,
    Err(_) => return HttpResponse::InternalServerError().body("Cannot get articles from database"),
  };

  let mut renderable_articles: Vec<RenderableArticle> = Vec::new();
  for article in &articles {
    renderable_articles.push(RenderableArticle::from_article(article));
  }

  let mut ctx = Context::new();
  ctx.insert("articles", &renderable_articles);
  // TODO handle rendering error
  let rendered = app_data.articles.templates.render("articles.html", &ctx).unwrap();
  HttpResponse::Ok().content_type("text/html").body(rendered)
}

/// Render the page for a specific article.
pub async fn render_article_view(app_data: web::Data<AppData>, id: web::Path<String>) -> impl Responder {
  let object_id = match bson::oid::ObjectId::with_string(id.into_inner().as_str()) {
    Ok(result) => result,
    Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
  };

  let article = match find_one_article_by_id(&app_data.get_ref().db, object_id).await {
    Ok(result) => match result {
      Some(value) => value,
      None => return HttpResponse::NotFound().body("article not found"),
    },
    Err(e) => return HttpResponse::InternalServerError().body(format!("Error finding object: {}", e)),
  };

  let mut ctx = Context::new();
  ctx.insert("title", article.title.as_str());
  ctx.insert("content", parse_markdown(&article.content).as_str());
  let rendered = app_data.articles.templates.render("article_view.html", &ctx).unwrap();
  HttpResponse::Ok().content_type("text/html").body(rendered)
}

/// Render the page containing the form to create a new article.
pub async fn render_new_article_view(app_data: web::Data<AppData>) -> impl Responder {
  let ctx = Context::new();
  let rendered = app_data.articles.templates.render("new_article.html", &ctx).unwrap();
  HttpResponse::Ok().content_type("text/html").body(rendered)
}

/// Render a page to redirect the user to another page.
/// Used to redirect to the article after the new article form post.
pub fn render_redirect_view(app_data: web::Data<AppData>, url: String) -> HttpResponse {
  let mut ctx = Context::new();
  ctx.insert("url", url.as_str());
  let rendered = app_data.articles.templates.render("redirect_view.html", &ctx).unwrap();
  HttpResponse::Ok().content_type("text/html").body(rendered)
}

/// New article post handler, which inserts the article in the Mongo database and redirect the user to the new article page.
pub async fn create_article_view(app_data: web::Data<AppData>, article: web::Form<EditableArticle>) -> impl Responder {
  let json_object_id = match insert_article(&app_data.get_ref().db, &article.into_inner()).await {
    Ok(obj_id) => obj_id,
    Err(e) => return HttpResponse::InternalServerError().body(format!("Error inserting object: {}", e)),
  };

  render_redirect_view(app_data, format!("{}", json_object_id))
}