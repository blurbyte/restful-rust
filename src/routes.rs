use std::sync::{Arc, Mutex};

use log::debug;
use warp::{filters::BoxedFilter, http::StatusCode, Filter, Rejection, Reply};

use crate::filters::{self, ListOptions};
use crate::schema::Game;

type Db = Arc<Mutex<Vec<Game>>>;

/// Provides RESTful API for games:
///
/// - `GET /games`: return JSON list of games
/// - `POST /games`: create a new game entry
/// - `PUT /games/:id`: update a specyfic game
/// - `DELETE /games/:id`: delete a specyfic game

pub fn games_routes(db: Db) -> BoxedFilter<(impl Reply,)> {
    let db = warp::any().map(move || db.clone());

    // Starting "games" API path
    let games = warp::path("games");

    // Make sure nothing comes after "games"
    // For example `GET /games` is OK, but `GET /games/23` is not
    let games_index = games.and(warp::path::end());

    // Refer to specyfic game by id
    let game_id = games.and(warp::path::param::<u64>()).and(warp::path::end());

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

    // `PUT /games/:id`
    let update = warp::put2()
        .and(game_id)
        .and(filters::json_body())
        .and(db.clone())
        .and_then(update_game);

    // `DELETE /games/:id`
    let delete = warp::delete2().and(game_id).and(db.clone()).and_then(delete_game);

    let api = list.or(create).or(update).or(delete);

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
            debug!("game of given id already exists: {}", game.id);

            Ok(StatusCode::BAD_REQUEST)
        }
        None => {
            games.push(new_game);
            Ok(StatusCode::CREATED)
        }
    }
}

// `PUT /games/:id`
fn update_game(id: u64, updated_game: Game, db: Db) -> Result<impl Reply, Rejection> {
    debug!("update existing game: id={}, game={:?}", id, updated_game);

    let mut games = db.lock().unwrap();

    match games.iter_mut().find(|game| game.id == id) {
        Some(game) => {
            *game = updated_game;

            Ok(StatusCode::OK)
        }
        None => {
            debug!("game of given id not found");

            Ok(StatusCode::NOT_FOUND)
        }
    }
}

// `DELETE /games/:id`
fn delete_game(id: u64, db: Db) -> Result<impl Reply, Rejection> {
    debug!("delete game: id={}", id);

    let mut games = db.lock().unwrap();

    let len = games.len();

    // Removes all games with given id
    games.retain(|game| game.id != id);

    // If games length was smaller specyfic game was found and removed
    let deleted = games.len() != len;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        debug!("game of given id not found");

        Ok(StatusCode::NOT_FOUND)
    }
}
