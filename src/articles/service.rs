use mongodb::{bson, Collection, Database, error};
use pulldown_cmark::{Parser, Options, html};
use ammonia::{clean};
use chrono::{Utc};

use futures::stream::StreamExt;

use crate::articles::model::{EditableArticle, Article};

/// Insert a document in Mongo.
pub async fn insert_document(collection: Collection, doc: bson::Document) -> Result<bson::oid::ObjectId, String> {
  match collection.insert_one(doc, None).await {
    Ok(inserted) => match bson::from_bson(inserted.inserted_id) {
      Ok(res) => Ok(res),
      Err(_) => Err("Cannot get inserted ObjectId".into()),
    },
    Err(err) => Err(format!("Error inserting: {}", err)),
  }
}

/// Get all the articles available in the mongo database.
pub async fn get_articles(db: &Database) -> Result<Vec<Article>, String> {
  let cursor = match db.collection("Articles").find(None, None).await {
    Ok(find_cursor) => find_cursor,
    Err(e) => return Err(format!("Error, cannot get documents: {}", e)),
  };

  let mut articles: Vec<Article> = Vec::new();

  let results: Vec<error::Result<bson::Document>> = cursor.collect().await;

  for result in results {
    match result {
      Ok(document) => {
        match bson::from_bson(bson::Bson::Document(document)) {
          Ok(article) => articles.push(article),
          Err(e) => return Err(format!("Error, cannot get documents: {}", e)),
        };
      },
      Err(e) => return Err(format!("Error, cannot get documents: {}", e)),
    }
  }

  Ok(articles)
}

/// Find and return an article with the specified id.
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

/// Insert an article in the database.
/// 
/// Perform the following operations:
/// - Convert article to bson document.
/// - Add the timestamps.
/// - Insert document in Mongo.
pub async fn insert_article(db: &Database, article: &EditableArticle) -> Result<bson::oid::ObjectId, String> {
  match bson::to_bson(article) {
    Ok(bson_object) => match bson_object {
      bson::Bson::Document(bson_doc) => {

        let mut inserted_bson = bson_doc.clone();
        let timestamp = Utc::now().timestamp();
        inserted_bson.insert("updated", timestamp);
        inserted_bson.insert("created", timestamp);
        insert_document(db.collection("Articles"), inserted_bson).await
      },
      _ => Err("Cannot create the bson document".into()),
    },
    Err(e) => Err(format!("Cannot create bson object: {}", e)),
  }
}

/// Parse the markdown content and return it as html content.
pub fn parse_markdown(markdown: &String) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  let parser = Parser::new_ext(markdown.as_str(), options);

  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  lysol_html(&html_output)
}

/// Sanitize html to ensure it does not abuse the user.
pub fn lysol_html(html: &String) -> String {
  clean(html.as_str())
}
