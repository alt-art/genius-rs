use crate::song::Song;

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    #[serde(rename = "type")]
    pub hit_type: String,
    pub result: Song
}