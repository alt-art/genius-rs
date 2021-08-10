use serde::Deserialize;
use std::collections::BTreeMap as Map;

use crate::user::{User, UserMetadata};
use crate::Body;

#[derive(Deserialize, Debug)]
pub struct AnnotationDescription {
    pub _type: String,
    pub annotator_id: u32,
    pub annotator_login: String,
    pub api_path: String,
    pub classification: String,
    pub fragment: String,
    pub id: u32,
    pub is_description: bool,
    pub path: String,
    pub range: Map<String, String>,
    pub song_id: u32,
    pub url: String,
    pub annotations: Vec<Annotation>,
}

#[derive(Deserialize, Debug)]
pub struct Annotation {
    pub api_path: String,
    pub body: Body,
    pub comment_count: Option<u32>,
    pub community: bool,
    pub has_voters: bool,
    pub id: u32,
    pub pinned: bool,
    pub share_url: String,
    pub state: String,
    pub url: String,
    pub verified: bool,
    pub votes_total: Option<u32>,
    pub current_user_metadata: UserMetadata,
    pub authors: Vec<AnnotationAuthor>,
}

#[derive(Deserialize, Debug)]
pub struct AnnotationAuthor {
    pub attribution: f32,
    pub user: User,
}
