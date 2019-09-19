// Common filters ment to be shared between many endpoints

use serde::Deserialize;
use warp::{filters::BoxedFilter, Filter};

use crate::schema::Game;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

// Optional query params to allow pagination
pub fn list_options() -> BoxedFilter<(ListOptions,)> {
    warp::query::<ListOptions>().boxed()
}

// Accept only JSON body and reject big payloads
pub fn json_body() -> BoxedFilter<(Game,)> {
    warp::body::content_length_limit(1024 * 32)
        .and(warp::body::json())
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::schema::Genre;
    use chrono::prelude::*;

    #[test]
    fn list_options_correct_query() {
        let filter = list_options();

        let options = warp::test::request()
            .path("/test?offset=5&limit=10")
            .filter(&filter)
            .unwrap();

        assert_eq!(
            options,
            ListOptions {
                offset: Some(5),
                limit: Some(10)
            }
        );
    }

    #[test]
    fn list_options_incorrect_query() {
        let filter = list_options();

        let options = warp::test::request()
            .path("/test?wrongparam1=5&wrongparam2=10")
            .filter(&filter)
            .unwrap();

        assert_eq!(
            options,
            ListOptions {
                offset: None,
                limit: None
            }
        );
    }

    #[test]
    fn json_body_correct() {
        let filter = json_body();

        let body = warp::test::request().header("content-length", 1024 * 16).body(r#"{"id":42,"title":"Test","rating":45,"genre":"ROLE_PLAYING","description":"Short description...","releaseDate":"2019-09-18T00:00:00"}"#).path("/test").filter(&filter).unwrap();

        assert_eq!(
            body,
            Game {
                id: 42,
                title: String::from("Test"),
                rating: 45,
                genre: Genre::RolePlaying,
                description: Some(String::from("Short description...")),
                release_date: NaiveDate::from_ymd(2019, 9, 18).and_hms(0, 0, 0),
            }
        );
    }

    #[test]
    #[should_panic]
    fn json_body_payload_too_large() {
        let filter = json_body();

        warp::test::request()
            .header("content-length", 1024 * 36)
            .path("/test")
            .filter(&filter)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn json_incorrect_body() {
        let filter = json_body();

        warp::test::request()
            .header("content-length", 1024 * 16)
            .body(r#"{"id": 1}"#)
            .path("/test")
            .filter(&filter)
            .unwrap();
    }
}
