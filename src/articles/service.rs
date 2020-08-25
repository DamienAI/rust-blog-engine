use mongodb::{bson, Collection, Database};
use actix_web::{web, HttpResponse, Responder};

use crate::articles::model::{EditableArticle, Article};
use crate::articles::views::render_redirect_view;

use crate::application::AppData;

pub async fn insert_document(collection: Collection, doc: bson::Document) -> Result<bson::oid::ObjectId, String> {
  match collection.insert_one(doc, None).await {
    Ok(inserted) => match bson::from_bson(inserted.inserted_id) {
      Ok(res) => Ok(res),
      Err(_) => Err("Cannot get inserted ObjectId".into()),
    },
    Err(err) => Err(format!("Error inserting: {}", err)),
  }
}

pub async fn find_one_article_by_id(db: &Database, id: bson::oid::ObjectId) -> Result<Option<Article>, String> {
  match db.collection("Articles").find_one(Some(bson::doc! {"_id": id}), None).await {
    Ok(mongo_result) => match mongo_result {
      Some(document) => match bson::from_bson(bson::Bson::Document(document)) {
        Ok(article) => Ok(Some(article)),
        Err(_) => Err("Error reversing bson object".into()),
      },
      None => Ok(None),
    },
    Err(e) => Err(format!("Error, cannot get document: {}", e)),
  }
}

pub async fn insert_article(db: &Database, article: &EditableArticle) -> Result<bson::oid::ObjectId, String> {
  match bson::to_bson(article) {
    Ok(bson_object) => match bson_object {
      bson::Bson::Document(bson_doc) => insert_document(db.collection("Articles"), bson_doc).await,
      _ => Err("Cannot create the bson document".into()),
    },
    Err(e) => Err(format!("Cannot create bson object: {}", e)),
  }
}

// TODO MOVE ?
pub async fn create_article(app_data: web::Data<AppData>, article: web::Form<EditableArticle>) -> impl Responder {
  let json_object_id = match insert_article(&app_data.get_ref().db, &article.into_inner()).await {
    Ok(obj_id) => obj_id,
    Err(e) => return HttpResponse::InternalServerError().body(format!("Error inserting object: {}", e)),
  };

  render_redirect_view(app_data, format!("{}", json_object_id))
}