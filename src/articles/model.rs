use serde::{Deserialize, Serialize};

use mongodb::{bson};

/// Actual article model as it is saved in the Mongo database.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
  #[serde(rename = "_id")]
  pub id: bson::oid::ObjectId,
  pub title: String,
  pub description: String,
  pub content: String,
  pub created: i64,
  pub updated: i64,
}

/// Specialization of an article for edition and creation
/// 
/// Basically it does not require id and do not accept any timestamps which are added by other means.
#[derive(Serialize, Deserialize)]
pub struct EditableArticle {
  pub title: String,
  pub description: String,
  pub content: String,
}

/// Specialization of the Article model for rendering in the html pages
/// 
#[derive(Serialize)]
pub struct RenderableArticle {
  pub id: String,
  pub title: String,
  pub description: String,
  pub content: String,
  pub published_str: String,
  pub edited_str: String,
}

impl RenderableArticle {
  /// Convert an article to a renderable article.
  /// 
  /// It provides strings with published since and last edited information.
  /// 
  pub fn from_article(article: &Article) -> RenderableArticle {
    RenderableArticle {
      id: article.id.to_string(),
      title: article.title.clone(),
      description: article.description.clone(),
      content: article.content.clone(),
      published_str: "2 days".to_string(),
      edited_str: "1 day".to_string(),
    }
  }
}

