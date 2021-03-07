use crate::checkvist_date;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct Checklist {
    pub id: u32,
    pub name: String,
    pub options: i32,
    pub public: bool,
    #[serde(rename = "markdown?")]
    pub markdown: bool,
    pub archived: bool,
    pub read_only: bool,
    pub user_count: i32,
    pub percent_completed: f32,
    pub task_count: i32,
    pub task_completed: i32,
    pub item_count: i32,
    pub tags: HashMap<String, bool>,
    pub tags_as_text: String,
    #[serde(with = "checkvist_date")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "checkvist_date")]
    pub user_updated_at: DateTime<Utc>,
}