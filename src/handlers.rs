// API handlers, the ends of each filter chain

use log::debug;
use std::convert::Infallible;
use warp::{http::StatusCode, Reply};

use crate::schema::{Db, Game, ListOptions};

// `GET /games`
// Returns JSON array of todos
// Allows pagination, for example: `GET /games?offset=10&limit=5`
pub async fn list_games(options: ListOptions, db: Db) -> Result<impl Reply, Infallible> {
    debug!("list all games");

    let games = db.lock().await;
    let games: Vec<Game> = games
        .clone()
        .into_iter()
        .skip(options.offset.unwrap_or(0))
        .take(options.limit.unwrap_or(std::usize::MAX))
        .collect();

    Ok(warp::reply::json(&games))
}

// `POST /games`
// Create new game entry with JSON body
pub async fn create_game(new_game: Game, db: Db) -> Result<impl Reply, Infallible> {
    debug!("create new game: {:?}", new_game);

    let mut games = db.lock().await;

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
pub async fn update_game(id: u64, updated_game: Game, db: Db) -> Result<impl Reply, Infallible> {
    debug!("update existing game: id={}, game={:?}", id, updated_game);

    let mut games = db.lock().await;

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
pub async fn delete_game(id: u64, db: Db) -> Result<impl Reply, Infallible> {
    debug!("delete game: id={}", id);

    let mut games = db.lock().await;

    let len = games.len();

    // Removes all games with given id
    games.retain(|game| game.id != id);

    // If games length was smaller that means specyfic game was found and removed
    let deleted = games.len() != len;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        debug!("game of given id not found");

        Ok(StatusCode::NOT_FOUND)
    }
}
