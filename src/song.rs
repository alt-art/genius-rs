use serde::Deserialize;

use crate::album::Album;
use crate::annotation::Referent;
use crate::user::{User, UserMetadata};
use crate::Body;

#[derive(Deserialize, Debug)]
pub struct Song {
    /// Number of annotations on this song.
    pub annotation_count: u32,
    /// Path of the API.
    pub api_path: String,
    /// Id of the song in apple music.
    /// > Only in `get_song`
    pub apple_music_id: Option<String>,
    /// URL of the song in apple music.
    /// > Only in `get_song`
    pub apple_music_player_url: Option<String>,
    /// Description of the music.
    /// > Only in `get_song`
    pub description: Option<Body>,
    /// HTML to embed the content in a web page.
    /// > Only in `get_song`
    pub embed_content: Option<String>,
    /// If has a video linked with this song.
    /// > Only in `get_song`
    pub featured_video: Option<bool>,
    /// Full song title is: "`name` by `author`".
    pub full_title: String,
    /// Header image with a smaller size.
    pub header_image_thumbnail_url: String,
    /// Header image.
    pub header_image_url: String,
    /// Id of the song.
    pub id: u32,
    /// Id of the user who requested the lyrics.
    pub lyrics_owner_id: u32,
    /// Lyrics state
    pub lyrics_state: String,
    /// Path where is in genius.com website.
    pub path: String,
    /// Number of pyongs in this song.
    pub pyongs_count: Option<u32>,
    /// The location of the recording.
    /// > Only in `get_song`
    pub recording_location: Option<String>,
    /// Release date of this song in ISO 8601 date format.
    /// > Only in `get_song`
    pub release_date: Option<String>,
    /// Release date in `{Month name} {day}, {year}` format.
    /// > Only in `get_song`
    pub release_date_for_display: Option<String>,
    /// Song art image with a smaller size.
    pub song_art_image_thumbnail_url: String,
    /// Song art image.
    pub song_art_image_url: String,
    /// Information about contribution and views.
    pub stats: SongStatus,
    /// Title of the song.
    pub title: String,
    /// Title but with the featured artist if it exists
    pub title_with_featured: String,
    /// Url of the song page.
    pub url: String,
    /// User permissions and interactions.
    /// > Only in `get_song`
    pub current_user_metadata: Option<UserMetadata>,
    /// Author of the song.
    pub primary_artist: Artist,
    /// Album of the song.
    /// > Only in `get_song`
    pub album: Option<Album>,
    /// People who worked in the music.
    /// > Only in `get_song`
    pub custom_performances: Option<Vec<SongPerformance>>,
    /// Description annotation.
    /// > Only in `get_song`
    pub description_annotation: Option<Referent>,
    /// Artists who featured in the song.
    /// > Only in `get_song`
    pub featured_artists: Option<Vec<Artist>>,
    /// Music platforms that host this song and its url.
    /// > Only in `get_song`
    pub media: Option<Vec<SongMedia>>,
    /// Artists who produced this song.
    /// > Only in `get_song`
    pub producer_artists: Option<Vec<Artist>>,
    /// Songs that somehow relate to this.
    /// > Only in `get_song`
    pub song_relationships: Option<Vec<SongRelationship>>,
    /// All Verified Annotation contributors.
    /// > Only in `get_song`
    pub verified_annotations_by: Option<Vec<User>>,
    /// All verified contributors
    /// > Only in `get_song`
    pub verified_contributors: Option<Vec<SongContributor>>,
    /// All Verified lyrics contributors.
    /// > Only in `get_song`
    pub verified_lyrics_by: Option<Vec<User>>,
    /// Composers
    /// > Only in `get_song`
    pub writer_artists: Option<Vec<Artist>>,
}

#[derive(Deserialize, Debug)]
pub struct SongContributor {
    pub contributions: Vec<String>,
    pub artist: Artist,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct SongRelationship {
    /// The type of relationship can be `samples`, `sampled_in`, `interpolates`, `interpolated_by`, `cover_of`, `covered_by`, `remix_of`, `remixed_by`, `live_version_of` and `performed_live_as`.
    pub relationship_type: String,
    /// Songs with this relationship type.
    pub songs: Vec<Option<Song>>,
}

#[derive(Deserialize, Debug)]
pub struct SongPerformance {
    pub label: String,
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, Debug)]
pub struct SongMedia {
    /// Spotify path of the song with `:` instead `/`, weird.
    pub native_uri: Option<String>,
    /// Soundcloud username.
    pub attribution: Option<String>,
    /// The song host provider `youtube`, `soundcloud` or `spotify`.
    pub provider: String,
    /// Youtube position of the video that starts the music.
    pub start: Option<u32>,
    /// Media type `video` or `audio`
    #[serde(rename = "type")]
    pub media_type: String,
    /// The url of the song in the media host.
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct SongStatus {
    /// Number of annotations accepted on this song.
    pub accepted_annotations: Option<u32>,
    /// Number of contributors.
    pub contributors: Option<u32>,
    /// Number of users who have earned iq.
    pub iq_earners: Option<u32>,
    /// Number of transcribers.
    pub transcribers: Option<u32>,
    /// Number of verified annotations.
    pub verified_annotations: Option<u32>,
    /// Number of unreviewed annotations.
    pub unreviewed_annotations: u32,
    /// If it's hot be careful with your hands.
    pub hot: bool,
    /// Number of page views
    pub pageviews: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    /// Path of the API.
    pub api_path: String,
    /// Artist header image.
    pub header_image_url: String,
    /// Artist id.
    pub id: u32,
    /// Artist image.
    pub image_url: String,
    /// Is this artist a meme?
    pub is_meme_verified: bool,
    /// If this artist is verified.
    pub is_verified: bool,
    /// Name of the artist.
    pub name: String,
    /// Url of the artist page.
    pub url: String,
    /// How much iq this artist has.
    pub iq: Option<u32>,
}
