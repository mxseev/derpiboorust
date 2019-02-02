use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::Id;

/// Links to various image sizes
#[derive(Debug, Deserialize)]
pub struct Representations {
    pub thumb_tiny: String,
    pub thumb_small: String,
    pub thumb: String,
    pub small: String,
    pub medium: String,
    pub large: String,
    pub tall: String,
    pub full: String,
}

/// Image model
#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub first_seen_at: DateTime<Utc>,
    pub score: i64,
    pub comment_count: u64,
    pub width: u64,
    pub height: u64,
    pub file_name: String,
    pub description: String,
    pub uploader: String,
    pub uploader_id: Option<Id>,
    pub image: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub faves: u64,
    pub tags: String,
    pub tag_ids: Vec<Id>,
    pub aspect_ratio: f64,
    pub original_format: String,
    pub mime_type: String,
    pub sha512_hash: String,
    pub orig_sha512_hash: Option<String>,
    pub source_url: String,
    pub representations: Representations,
    pub is_rendered: bool,
    pub is_optimized: bool,
    pub spoilered: Option<bool>,
}
