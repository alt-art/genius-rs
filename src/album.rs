use serde::Deserialize;

use crate::annotation::AnnotationDescription;
use crate::song::{Artist, SongPerformance};
use crate::user::UserMetadata;

#[derive(Deserialize, Debug)]
pub struct Album {
    pub api_path: String,
    pub comment_count: Option<u32>,
    pub cover_art_url: String,
    pub custom_header_image_url: Option<String>,
    pub full_title: String,
    pub header_image_url: Option<String>,
    pub id: u32,
    pub lock_state: Option<String>,
    pub name: String,
    pub pyongs_count: Option<u32>,
    pub release_date: Option<String>,
    pub release_date_components: Option<Date>,
    pub url: String,
    pub current_user_metadata: Option<UserMetadata>,
    pub song_pageviews: Option<u32>,
    pub artist: Artist,
    pub cover_arts: Option<Vec<CoverArt>>,
    pub description_annotation: Option<AnnotationDescription>,
    pub song_performances: Option<Vec<SongPerformance>>,
}

#[derive(Deserialize, Debug)]
pub struct CoverArt {
    pub annotated: bool,
    pub api_path: String,
    pub id: u32,
    pub image_url: String,
    pub thumbnail_image_url: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}
