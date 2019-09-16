use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::validators;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: u64,
    pub title: String,
    #[serde(with = "validators::validate_game_rating")]
    pub rating: u8,
    pub genre: Genre,
    pub description: Option<String>,
    pub release_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Genre {
    RolePlaying,
    Strategy,
    Shooter,
}
