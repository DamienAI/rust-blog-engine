use serde::{Deserialize, Serialize};

use mongodb::{bson};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
  #[serde(rename = "_id")]
  pub id: bson::oid::ObjectId,
  pub title: String,
  pub description: String,
  pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditableArticle {
  pub title: String,
  pub description: String,
  pub content: String,
}

