use serde::Deserialize;
use std::collections::BTreeMap as Map;

#[derive(Deserialize, Debug)]
pub struct User {
    /// User api path
    pub api_path: String,
    /// User profile photo
    pub avatar: Map<String, AvatarImage>,
    /// User header image
    pub header_image_url: String,
    /// User role human readable.
    pub human_readable_role_for_display: Option<String>,
    /// User id.
    pub id: u32,
    /// User iq.
    pub iq: Option<u32>,
    /// Username.
    pub login: String,
    /// User name.
    pub name: String,
    /// User role.
    pub role_for_display: Option<String>,
    /// User page url.
    pub url: String,
    /// User permissions and interactions.
    pub current_user_metadata: UserMetadata,
}

#[derive(Deserialize, Debug)]
pub struct AvatarImage {
    /// Image url.
    pub url: String,
    /// Width and height.
    pub bounding_box: Map<String, u32>,
}

#[derive(Deserialize, Debug)]
pub struct UserMetadata {
    /// Permissions you have.
    pub permissions: Vec<String>,
    /// Permissions you don't have.
    pub excluded_permissions: Vec<String>,
    pub interactions: Interactions,
}

#[derive(Deserialize, Debug)]
pub struct Interactions {
    cosign: Option<bool>,
    pyong: Option<bool>,
    vote: Option<u32>,
}
