use mongodb::Database;
use crate::articles::views::ArticlesAppData;

pub struct AppData
{
  pub db: Database,
  pub articles: ArticlesAppData,
}
