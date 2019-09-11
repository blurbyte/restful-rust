use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::utils::validate_rating;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: u64,
    pub title: String,
    #[serde(with = "validate_rating")]
    pub rating: u8,
    pub genre: Genre,
    pub description: String,
    pub release_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Genre {
    RolePlaying,
    Strategy,
    Shooter,
}
