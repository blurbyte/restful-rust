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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::schema::Genre;
    use chrono::prelude::*;

    // Mocked dataset for each test
    fn mocked_db() -> Db {
        Arc::new(Mutex::new(vec![
            Game {
                id: 1,
                title: String::from("Crappy title"),
                rating: 35,
                genre: Genre::RolePlaying,
                description: Some(String::from("Test description...")),
                release_date: NaiveDate::from_ymd(2011, 9, 22).and_hms(0, 0, 0),
            },
            Game {
                id: 2,
                title: String::from("Decent game"),
                rating: 84,
                genre: Genre::Strategy,
                description: None,
                release_date: NaiveDate::from_ymd(2014, 3, 11).and_hms(0, 0, 0),
            },
        ]))
    }

    #[test]
    fn get_list_of_games_200() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().path("/games").reply(&filter);

        assert_eq!(res.status(), 200);

        let expected_json_body = r#"[{"id":1,"title":"Crappy title","rating":35,"genre":"ROLE_PLAYING","description":"Test description...","releaseDate":"2011-09-22T00:00:00"},{"id":2,"title":"Decent game","rating":84,"genre":"STRATEGY","description":null,"releaseDate":"2014-03-11T00:00:00"}]"#;
        assert_eq!(res.body(), expected_json_body);
    }

    #[test]
    fn get_list_of_games_with_options_200() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().path("/games?offset=1&limit=5").reply(&filter);

        assert_eq!(res.status(), 200);

        let expected_json_body = r#"[{"id":2,"title":"Decent game","rating":84,"genre":"STRATEGY","description":null,"releaseDate":"2014-03-11T00:00:00"}]"#;
        assert_eq!(res.body(), expected_json_body);
    }

    #[test]
    fn get_empty_list_with_offset_overshot_200() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().path("/games?offset=5&limit=5").reply(&filter);

        assert_eq!(res.status(), 200);

        let expected_json_body = r#"[]"#;
        assert_eq!(res.body(), expected_json_body);
    }

    #[test]
    fn get_incorrect_options_400() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().path("/games?offset=a&limit=b").reply(&filter);

        assert_eq!(res.status(), 400);
    }

    #[test]
    fn get_wrong_path_405() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().path("/games/42").reply(&filter);

        assert_eq!(res.status(), 405);
    }

    #[test]
    fn post_json_201() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let json_payload = r#"{"id":3,"title":"Another game","rating":65,"genre":"STRATEGY","description":null,"releaseDate":"2016-03-11T00:00:00"}"#;
        let res = warp::test::request()
            .method("POST")
            .header("content-length", 1024 * 16)
            .body(&json_payload)
            .path("/games")
            .reply(&filter);

        assert_eq!(res.status(), 201);
        assert_eq!(db.lock().unwrap().len(), 3);
    }

    #[test]
    fn post_too_long_content_413() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let json_payload = r#"{"id":3,"title":"Another game","rating":65,"genre":"STRATEGY","description":null,"releaseDate":"2016-03-11T00:00:00"}"#;
        let res = warp::test::request()
            .method("POST")
            .header("content-length", 1024 * 36)
            .body(&json_payload)
            .path("/games")
            .reply(&filter);

        assert_eq!(res.status(), 413);
    }

    #[test]
    fn post_wrong_payload_400() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request()
            .method("POST")
            .header("content-length", 1024 * 16)
            .body(&r#"{"id":4}"#)
            .path("/games")
            .reply(&filter);

        assert_eq!(res.status(), 400);
    }

    #[test]
    fn post_wrong_path_405() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().method("POST").path("/games/42").reply(&filter);

        assert_eq!(res.status(), 405);
    }

    #[test]
    fn put_json_200() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let json_payload = r#"{"id":2,"title":"Decent game","rating":84,"genre":"STRATEGY","description":"New description","releaseDate":"2014-03-11T00:00:00"}"#;
        let res = warp::test::request()
            .method("PUT")
            .header("content-length", 1024 * 16)
            .body(&json_payload)
            .path("/games/2")
            .reply(&filter);

        assert_eq!(res.status(), 200);

        let db = db.lock().unwrap();
        let description = db[1].description.as_ref().unwrap();
        assert_eq!(description, "New description");
    }

    #[test]
    fn put_wrong_id_404() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let json_payload = r#"{"id":2,"title":"Decent game","rating":84,"genre":"STRATEGY","description":"New description","releaseDate":"2014-03-11T00:00:00"}"#;
        let res = warp::test::request()
            .method("PUT")
            .header("content-length", 1024 * 16)
            .body(&json_payload)
            .path("/games/42")
            .reply(&filter);

        assert_eq!(res.status(), 404);
    }

    #[test]
    fn put_wrong_payload_400() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request()
            .method("PUT")
            .header("content-length", 1024 * 16)
            .body(&r#"{"id":2"#)
            .path("/games/2")
            .reply(&filter);

        assert_eq!(res.status(), 400);
    }

    #[test]
    fn put_too_long_content_413() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request()
            .method("PUT")
            .header("content-length", 1024 * 36)
            .path("/games/2")
            .reply(&filter);

        assert_eq!(res.status(), 413);
    }

    #[test]
    fn delete_wrong_id_404() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().method("DELETE").path("/games/42").reply(&filter);

        assert_eq!(res.status(), 404);
    }

    #[test]
    fn delete_game_204() {
        let db = mocked_db();
        let filter = games_routes(db.clone());

        let res = warp::test::request().method("DELETE").path("/games/1").reply(&filter);

        assert_eq!(res.status(), 204);
        assert_eq!(db.lock().unwrap().len(), 1);
    }
}
