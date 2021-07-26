use serde::{Deserialize};

use crate::annotation::AnnotationDescription;
use crate::user::{User, UserMetadata};
use crate::Body;

#[derive(Deserialize, Debug)]
pub struct Song {
    pub annotation_count: u32,
    pub api_path: String,
    pub apple_music_id: Option<String>,
    pub apple_music_player_url: Option<String>,
    pub description: Option<Body>,
    pub embed_content: Option<String>,
    pub featured_video: Option<bool>,
    pub full_title: String,
    pub header_image_thumbnail_url: String,
    pub header_image_url: String,
    pub id: u32,
    pub lyrics_owner_id: u32,
    pub lyrics_state: String,
    pub path: String,
    pub pyongs_count: Option<u32>,
    pub recording_location: Option<String>,
    pub release_date: Option<String>,
    pub release_date_for_display: Option<String>,
    pub song_art_image_thumbnail_url: String,
    pub song_art_image_url: String,
    pub stats: SongStatus,
    pub title: String,
    pub title_with_featured: String,
    pub url: String,
    pub current_user_metadata: Option<UserMetadata>,
    pub song_art_primary_color: Option<String>,
    pub song_art_secondary_color: Option<String>,
    pub song_art_text_color: Option<String>,
    pub primary_artist: Artist,
    pub album: Option<Album>,
    pub custom_performances: Option<Vec<SongPerformance>>,
    pub description_annotation: Option<AnnotationDescription>,
    pub featured_artists: Option<Vec<Artist>>,
    pub media: Option<Vec<SongMedia>>,
    pub producer_artists: Option<Vec<Artist>>,
    pub song_relationships: Option<Vec<SongRelationship>>,
    pub verified_annotations_by: Option<Vec<User>>,
    pub verified_contributors: Option<Vec<SongContributor>>,
    pub verified_lyrics_by: Option<Vec<User>>,
    pub writer_artists: Option<Vec<Artist>>,
}

#[derive(Deserialize, Debug)]
pub struct SongContributor {
    pub contributions: Vec<String>,
    pub artist: Artist,
    pub user: User
}

#[derive(Deserialize, Debug)]
pub struct SongRelationship {
    pub relationship_type: String,
    pub songs: Vec<Option<Song>>
}



#[derive(Deserialize, Debug)]
pub struct SongPerformance {
    label: String,
    artists: Vec<Artist>
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub api_path: String,
    pub cover_art_url: String,
    pub full_title: String,
    pub id: u32,
    pub name: String,
    pub url: String,
    pub artist: Artist,
}

#[derive(Deserialize, Debug)]
pub struct SongMedia {
    pub native_uri: Option<String>,
    pub attribution: Option<String>,
    pub provider: String,
    pub start: Option<u32>,
    #[serde(rename = "type")]
    pub media_type: String,
    pub url: String
}

#[derive(Deserialize, Debug)]
pub struct SongStatus {
    pub accepted_annotations: Option<u32>,
    pub contributors: Option<u32>,
    pub iq_earners: Option<u32>,
    pub transcribers: Option<u32>,
    pub verified_annotations: Option<u32>,
    pub unreviewed_annotations: u32,
    pub hot: bool,
    pub pageviews: Option<u32>
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