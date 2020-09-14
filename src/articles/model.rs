use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDateTime, DateTime};
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

fn get_elapsed_time_str(timestamp: i64) -> String {
  let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
  let duration = Utc::now().signed_duration_since(dt);
  if duration.num_days() > 0 {
    return format!("{} days", duration.num_days())
  } else if duration.num_hours() > 0 {
    return format!("{} hours", duration.num_hours())
  }

  format!("{} minutes", duration.num_minutes())
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
      published_str: get_elapsed_time_str(article.created),
      edited_str: get_elapsed_time_str(article.updated),
    }
  }
}

