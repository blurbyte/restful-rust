use std::boxed::Box;
use std::error::Error;
use std::sync::{Arc, Mutex};

use chrono::prelude::*;
use warp::Filter;

mod filters;
mod routes;
mod schema;
mod validators;

use crate::schema::{Game, Genre};

pub fn run() -> Result<(), Box<dyn Error>> {
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

    let games = routes::games_routes(db);

    let routes = games.with(warp::log("restful_rust"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080));

    Ok(())
}
