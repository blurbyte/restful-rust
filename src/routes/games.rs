use std::sync::{Arc, Mutex};

use chrono::prelude::*;
use log::debug;
use warp::{filters::BoxedFilter, Filter, Reply};

use crate::filters::{self, ListOptions};
use crate::schema::{Game, Genre};

type Db = Arc<Mutex<Vec<Game>>>;

/// Provides RESTful API for games:
///
/// - `GET /games`: return JSON list of games

pub fn games_route() -> BoxedFilter<(impl Reply,)> {
    // For presentation purposes keep mocked data in in-memory structure
    // In real life scenario connection with regular database would be established
    let db = Arc::new(Mutex::new(
        vec![
            Game {
                id: 1,
                title: String::from("Dark Souls"),
                rating: 91,
                genre: Genre::RolePlaying,
                description: Some(String::from("Takes place in the fictional kingdom of Lordran, where players assume the role of a cursed undead character who begins a pilgrimage to discover the fate of their kind.")),
                release_date: NaiveDate::from_ymd(2011, 9, 22).and_hms(0, 0, 0),
            },
            Game {
                id: 2,
                title: String::from("Dark Souls 2"),
                rating: 87,
                genre: Genre::RolePlaying,
                description: None,
                release_date: NaiveDate::from_ymd(2014, 3, 11).and_hms(0, 0, 0),
            },
            Game {
                id: 3,
                title: String::from("Dark Souls 3"),
                rating: 89,
                genre: Genre::RolePlaying,
                description: Some(String::from("The latest chapter in the series with its trademark sword and sorcery combat and rewarding action RPG gameplay.")),
                release_date: NaiveDate::from_ymd(2016, 3, 24).and_hms(0, 0, 0),
            },
        ]
    ));

    let db = warp::any().map(move || db.clone());

    // Starting "games" API path
    let games = warp::path("games");

    // Make sure nothing comes after "games"
    // For example `GET /games` is OK, but `GET /games/23` is not
    let games_index = games.and(warp::path::end());

    // `GET /games`
    let list = warp::get2().and(games_index).and(filters::list_options()).and(db.clone()).map(list_games);

    list.boxed()
}

// `GET /games`
// Allows pagination, for example: `GET /games?offset=10&limit=5`
fn list_games(options: ListOptions, db: Db) -> impl Reply {
    debug!("list all games");

    let games = db.lock().unwrap();
    let games: Vec<Game> = games
        .clone()
        .into_iter()
        .skip(options.offset.unwrap_or(0))
        .take(options.limit.unwrap_or(std::usize::MAX))
        .collect();

    warp::reply::json(&games)
}
