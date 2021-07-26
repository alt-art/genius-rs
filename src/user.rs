use serde::{Deserialize};
use std::collections::BTreeMap as Map;

#[derive(Deserialize, Debug)]
pub struct User {
    pub api_path: String,
    pub avatar: Map<String, AvatarImage>,
    pub header_image_url: String,
    pub human_readable_role_for_display: String,
    pub id: u32,
    pub iq: Option<u32>,
    pub login: String,
    pub name: String,
    pub role_for_display: String,
    pub url: String,
    pub current_user_metadata: UserMetadata
}

#[derive(Deserialize, Debug)]
pub struct AvatarImage {
    pub url: String,
    pub bounding_box: Map<String,u32>
}

#[derive(Deserialize, Debug)]
pub struct UserMetadata {
    pub permissions: Vec<String>,
    pub excluded_permissions: Vec<String>,
    pub interactions: Interactions
}

#[derive(Deserialize, Debug)]
pub struct Interactions {
    cosign: Option<bool>,
    pyong: Option<bool>,
    vote: Option<u32>
}