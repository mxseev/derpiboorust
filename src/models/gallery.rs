use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Gallery model
#[derive(Debug, Deserialize)]
pub struct Gallery {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub spoiler_warning: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub creator_id: u64,
    pub watcher_count: u64,
    pub image_count: u64,
    pub image_ids: Option<Vec<u64>>,
}
