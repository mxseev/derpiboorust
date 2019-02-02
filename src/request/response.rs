//! API methods responses.

use serde::Deserialize;

use crate::models::{Gallery, Image};

/// [Search](crate::request::Search) response.
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub search: Vec<Image>,
    pub total: u64,
}

/// [Lists](crate::request::Lists) response.
#[derive(Debug, Deserialize)]
pub struct ListsResponse {
    pub top_scoring: Vec<Image>,
    pub top_commented: Vec<Image>,
    pub all_time_top_scoring: Vec<Image>,
}

/// [Image](crate::request::Image) response.
pub type ImageResponse = Image;

/// [Gallery](crate::request::Gallery) response.
#[derive(Debug, Deserialize)]
pub struct GalleryResponse {
    pub gallery: Gallery,
    pub images: Vec<Image>,
}

/// [Galleries](crate::request::Galleries) response.
pub type GalleriesResponse = Vec<Gallery>;

/// [Images](crate::request::Images) and [Watched](crate::request::Watched) response.
#[derive(Debug, Deserialize)]
pub struct ImagesResponse {
    pub images: Vec<Image>,
}
