use chrono::prelude::*;
use log::debug;
use warp::{filters::BoxedFilter, Filter, Reply};

use crate::schema::{Game, Genre};

pub fn games_route() -> BoxedFilter<(impl Reply,)> {
    // Starting "games" API path
    let games = warp::path("games");

    // Make sure nothing comes after "games"
    // For example `GET /games` is OK, but `GET /games/23` is not
    let games_index = games.and(warp::path::end());

    // GET /games
    let list = warp::get2().and(games_index).map(list_games);

    list.boxed()
}

// GET /games
fn list_games() -> impl Reply {
    debug!("List of games!");

    let games = vec![
        Game {
            id: 1,
            title: String::from("Dark Souls"),
            rating: 91,
            genre: Genre::RolePlaying,
            description: String::from("Nice game"),
            release_date: NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0),
        },
        Game {
            id: 2,
            title: String::from("Dark Souls 2"),
            rating: 85,
            genre: Genre::RolePlaying,
            description: String::from("Nice game 2"),
            release_date: NaiveDate::from_ymd(2014, 7, 8).and_hms(0, 0, 0),
        },
    ];

    warp::reply::json(&games)
}
