// Common filters ment to be shared between many endpoints

use serde::Deserialize;
use warp::{filters::BoxedFilter, Filter};

#[derive(Deserialize, Debug)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

// Optional query params to allow pagination
pub fn list_options() -> BoxedFilter<(ListOptions,)> {
    warp::query::<ListOptions>().boxed()
}
