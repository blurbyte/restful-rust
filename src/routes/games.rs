use std::sync::{Arc, Mutex};

use chrono::prelude::*;
use log::debug;
use warp::{filters::BoxedFilter, Filter, Reply};

use crate::schema::{Game, Genre};

type Db = Arc<Mutex<Vec<Game>>>;

pub fn games_route() -> BoxedFilter<(impl Reply,)> {
    // For presentation purposes keep mocked data in in-memory structure
    let db = Arc::new(Mutex::new(
        vec![
            Game {
                id: 1,
                title: String::from("Dark Souls"),
                rating: 91,
                genre: Genre::RolePlaying,
                description: String::from("Takes place in the fictional kingdom of Lordran, where players assume the role of a cursed undead character who begins a pilgrimage to discover the fate of their kind.."),
                release_date: NaiveDate::from_ymd(2011, 9, 22).and_hms(0, 0, 0),
            },
            Game {
                id: 2,
                title: String::from("Dark Souls 2"),
                rating: 87,
                genre: Genre::RolePlaying,
                description: String::from("It keeps in line with its predecessors in the Souls series by providing players with a deep but concealed story that must be pieced together via NPC dialogue, item descriptions, appearance, and geographic clues."),
                release_date: NaiveDate::from_ymd(2014, 3, 11).and_hms(0, 0, 0),
            },
        ]
    ));

    let db = warp::any().map(move || db.clone());

    // Starting "games" API path
    let games = warp::path("games");

    // Make sure nothing comes after "games"
    // For example `GET /games` is OK, but `GET /games/23` is not
    let games_index = games.and(warp::path::end());

    // GET /games
    let list = warp::get2().and(games_index).and(db.clone()).map(list_games);

    list.boxed()
}

// GET /games
fn list_games(db: Db) -> impl Reply {
    debug!("list all games");

    let games: Vec<Game> = db.lock().unwrap().to_vec();

    warp::reply::json(&games)
}
