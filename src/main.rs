use std::env;
use warp::Filter;

mod custom_filters;
mod handlers;
mod routes;
mod schema;
mod validators;

#[tokio::main]
async fn main() {
    // Show debug logs by default by setting `RUST_LOG=restful_rust=debug`
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "restful_rust=debug");
    }
    pretty_env_logger::init();

    let db = schema::example_db();

    let api = routes::games_routes(db);

    let routes = api.with(warp::log("restful_rust"));

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
