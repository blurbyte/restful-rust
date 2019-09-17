// Common filters ment to be shared between many endpoints

use serde::Deserialize;
use warp::{filters::BoxedFilter, Filter};

use crate::schema::Game;

#[derive(Deserialize, Debug)]
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
