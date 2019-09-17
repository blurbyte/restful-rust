use std::env;

use restful_rust;

fn main() {
    // Show debug logs by default by setting `RUST_LOG=restful_rust=debug`
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "restful_rust=debug");
    }
    pretty_env_logger::init();

    if let Err(error) = restful_rust::run() {
        eprintln!("{}", error);
    }
}
