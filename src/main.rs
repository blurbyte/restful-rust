use std::env;

use warp::Filter;

use restful_rust::routes::games;

fn main() {
    // Show debug logs by default by setting `RUST_LOG=restful_rust=debug`
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "restful_rust=debug");
    }
    pretty_env_logger::init();

    let games = games::games_route();

    let routes = games.with(warp::log("restful_rust"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080));
}
