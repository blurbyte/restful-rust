# RESTful Rust

[![Build Status](https://travis-ci.org/blurbyte/restful-rust.svg?branch=master)](https://travis-ci.org/blurbyte/restful-rust)

**RESTful Rust** is straightforward REST API example written in Rust. It shows how to *implement* and *test* GET, POST, PUT and DELETE methods with amazing [Warp](https://crates.io/crates/warp) web server framework.

### Getting started

To run the project locally:

1. install **rustup** by following the [instructions](https://www.rust-lang.org/tools/install)
2. add **clippy** (collection of lints) and **rustfmt** (code formatter) by running `rustup component add clippy` and `rustup component add rustfmt` accordingly
2. clone this repository `git clone https://github.com/blurbyte/restful-rust.git`
3. to **start an API** enter project's directory and run `cargo run`
4. run tests with `cargo test`
5. lint code with `cargo clippy` and format it with `cargo fmt`
6. run `cargo build --release` command to generate single optimized binary

### Dependencies overview

Dependency | Description
--- | ---
[warp](https://crates.io/crates/warp) | Composable web server framework with powerful *filters* system
[serde](https://crates.io/crates/serde) | Library for *serializing* and *deserializing* data structures
[chrono](https://crates.io/crates/chrono) | Date and time utilities
[log](https://crates.io/crates/log) + [pretty_env_logger](https://crates.io/crates/pretty_env_logger) | Simple logger (by default enabled in *debug* mode)

### Available endpoints

List of API routes with associated REST verbs:

* http://localhost:8080/games - GET, POST
* http://localhost:8080/games/:id - PUT, DELETE

### Testing RESTful API

Nowadays there are many great tools which make testing API easy, such as [Postman](https://www.getpostman.com/) or [Insomnia](https://insomnia.rest).

Just enter one of the available endpoints with appropriate HTTP method selected:

<img src="https://user-images.githubusercontent.com/20565536/65247241-584da300-daf0-11e9-90f4-fa8837ea976d.png" alt="Testing POST method with Insomnia REST client" width="600">

And watch a console / terminal for detailed logs:

<img src="https://user-images.githubusercontent.com/20565536/65247443-ae224b00-daf0-11e9-908c-e6fe02d574c9.png" alt="Terminal logs after each HTTP request" width="600">
