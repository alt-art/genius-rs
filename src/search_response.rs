use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    meta: Meta,
    pub response: SearchResult
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub status: u8
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub hits: Vec<Hit>
}

#[derive(Deserialize, Debug)]
pub struct Hit {
    highlights: [String;0],
    index: String,
    r#type: String,
    pub result: Song
}

#[derive(Deserialize, Debug)]
pub struct Song {
    pub annotation_count: u32,
    pub api_path: String,
    pub full_title: String,
    pub header_image_thumbnail_url: String,
    pub header_image_url: String,
    pub id: u32,
    pub lyrics_owner_id: u32,
    pub lyrics_state: String,
    pub path: String,
    pub pyongs_count: u32,
    pub song_art_image_thumbnail_url: String,
    pub song_art_image_url: String,
    pub song_art_primary_color: Option<String>,
    pub song_art_secondary_color: Option<String>,
    pub song_art_text_color: Option<String>,
    pub stats: SongStatus,
    pub title: String,
    pub title_with_featured: String,
    pub url: String,
    pub primary_artist: Artist
}

#[derive(Deserialize, Debug)]
pub struct SongStatus {
    pub unreviewed_annotations: u32,
    pub hot: bool,
    pub pageviews: u32
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub api_path: String,
    pub header_image_url: String,
    pub id: u32,
    pub image_url: String,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: String,
    pub url: String,
    pub iq: Option<u32>,
}