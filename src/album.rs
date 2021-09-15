use serde::Deserialize;

use crate::annotation::Referent;
use crate::song::{Artist, SongPerformance};
use crate::user::UserMetadata;

#[derive(Deserialize, Debug)]
pub struct Album {
    /// Path of the API.
    pub api_path: String,
    /// Number of comments.
    /// > Only in `get_album`
    pub comment_count: Option<u32>,
    /// Cover art of the album.
    pub cover_art_url: String,
    /// Custom header image.
    /// > Only in `get_album`
    pub custom_header_image_url: Option<String>,
    /// Full album title is: "`name` by `author`".
    pub full_title: String,
    /// Header image.
    /// > Only in `get_album`
    pub header_image_url: Option<String>,
    /// Id of the album.
    pub id: u32,
    /// > Only in `get_album`
    pub lock_state: Option<String>,
    /// Name of the album.
    pub name: String,
    /// Number of pyongs in this album.
    /// > Only in `get_album`
    pub pyongs_count: Option<u32>,
    /// Release date of the album in ISO 8601 date format.
    /// > Only in `get_album`
    pub release_date: Option<String>,
    /// Release date of the album in struct format [`Date`].
    /// > Only in `get_album`
    pub release_date_components: Option<Date>,
    /// Url of the album page.
    pub url: String,
    /// Current user metadata have all the permissions of the user to vote, edit etc. and some others stuffs.
    /// > Only in `get_album`
    pub current_user_metadata: Option<UserMetadata>,
    /// Number of views in the album page.
    /// > Only in `get_album`
    #[serde(rename = "song_pageviews")]
    pub album_pageviews: Option<u32>,
    /// Album artist.
    pub artist: Artist,
    /// All cover arts of the album.
    /// > Only in `get_album`
    pub cover_arts: Option<Vec<CoverArt>>,
    /// Annotations in this album.
    /// > Only in `get_album`
    pub description_annotation: Option<Referent>,
    /// All the people who worked on this album.
    /// > Only in `get_album`
    pub song_performances: Option<Vec<SongPerformance>>,
}

#[derive(Deserialize, Debug)]
pub struct CoverArt {
    /// If this art have annotations.
    pub annotated: bool,
    /// Path of the API
    pub api_path: String,
    /// Id of cover art.
    pub id: u32,
    /// Image of the art.
    pub image_url: String,
    /// Thumbnail image of the art.
    pub thumbnail_image_url: String,
    /// Page of the art.
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}
