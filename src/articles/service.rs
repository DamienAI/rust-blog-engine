use mongodb::{bson, Collection, Database};
use pulldown_cmark::{Parser, Options, html};
use ammonia::{clean};

use crate::articles::model::{EditableArticle, Article};

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

pub fn parse_markdown(markdown: &String) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  let parser = Parser::new_ext(markdown.as_str(), options);

  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  lysol_html(&html_output)
}

pub fn lysol_html(html: &String) -> String {
  clean(html.as_str())
}