use std::sync::{Arc, Mutex};

use log::debug;
use warp::{filters::BoxedFilter, http::StatusCode, Filter, Rejection, Reply};

use crate::filters::{self, ListOptions};
use crate::schema::Game;

type Db = Arc<Mutex<Vec<Game>>>;

/// Provides RESTful API for games:
///
/// - `GET /games`: return JSON list of games
/// - `POST /games`: create new game entry

pub fn games_routes(db: Db) -> BoxedFilter<(impl Reply,)> {
    let db = warp::any().map(move || db.clone());

    // Starting "games" API path
    let games = warp::path("games");

    // Make sure nothing comes after "games"
    // For example `GET /games` is OK, but `GET /games/23` is not
    let games_index = games.and(warp::path::end());

    // `GET /games`
    let list = warp::get2()
        .and(games_index)
        .and(filters::list_options())
        .and(db.clone())
        .map(list_games);

    // `POST /games`
    let create = warp::post2()
        .and(games_index)
        .and(filters::json_body())
        .and(db.clone())
        .and_then(create_game);

    let api = list.or(create);

    api.boxed()
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

// `POST /games`
// Create new game entry with JSON body
fn create_game(new_game: Game, db: Db) -> Result<impl Reply, Rejection> {
    debug!("create new game: {:?}", new_game);

    let mut games = db.lock().unwrap();

    match games.iter().find(|game| game.id == new_game.id) {
        Some(game) => {
            debug!("game of given ID already exists: {}", game.id);

            Ok(StatusCode::BAD_REQUEST)
        }
        None => {
            games.push(new_game);
            Ok(StatusCode::CREATED)
        }
    }
}
