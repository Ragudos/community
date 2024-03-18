use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::RwLock;

use controllers::rate_limiter::RateLimiter;
use rocket;
use rocket::figment::value::{Map, Value};
use rocket::figment::{Figment, Profile, Provider};
use rocket::fs::FileServer;
use rocket::Build;
use rocket::Rocket;
use rocket::{catchers as rocket_catchers, routes as rocket_routes};

use crate::env::{Env, Environment};
use crate::helpers::db;
use crate::helpers::get_environment;
use crate::helpers::handlebars;

pub mod catchers;
pub mod controllers;
pub mod env;
pub mod helpers;
pub mod models;
pub mod responders;
pub mod routes;

pub fn rocket_from_config(figment: Figment) -> Rocket<Build> {
    let rate_limit_capacity = figment
        .data()
        .unwrap()
        .get(&Profile::Global)
        .unwrap()
        .get("rate-limit-capacity")
        .unwrap()
        .to_num()
        .unwrap()
        .to_u32()
        .unwrap();
    let rate_limiter = RateLimiter {
        capacity: AtomicU32::new(rate_limit_capacity),
        time_accumulator_started: RwLock::new(time::OffsetDateTime::now_utc()),
        did_time_accumulator_start: AtomicBool::new(false),
        requests: AtomicU32::new(0),
    };
    let rocket = rocket::custom(figment)
        .attach(db::stage())
        .attach(handlebars::register())
        .manage(rate_limiter)
        .manage(Environment {
            environment: get_environment().into(),
        })
        .register(
            "/",
            rocket_catchers![
                catchers::unprocessable_entity,
                catchers::not_found,
                catchers::internal_server_error
            ],
        )
        .mount("/", rocket_routes![routes::page])
        .mount(
            "/auth",
            rocket_routes![
                routes::auth::logged_in,
                routes::auth::logged_out,
                routes::auth::login::page,
                routes::auth::register::page
            ],
        )
        .mount(
            "/auth/api",
            rocket_routes![
                routes::auth::api::login::post,
                routes::auth::api::register::post,
                routes::auth::api::logout::delete,
                routes::auth::api::login::logged_in,
                routes::auth::api::register::logged_in
            ],
        )
        .mount(
            "/community",
            rocket_routes![
                routes::community::logged_out,
                routes::community::page,
                routes::community::uid::page,
                routes::community::uid::settings::page,
                routes::community::uid::about::page,
                routes::community::uid::members::page,
            ],
        )
        .mount(
            "/community/api",
            rocket_routes![
                routes::community::api::logged_out,
                routes::community::api::get,
                routes::community::api::uid::get
            ],
        )
        .mount(
            "/create",
            rocket_routes![routes::create::logged_out, routes::create::community::page],
        )
        .mount(
            "/create/api",
            rocket_routes![routes::create::api::logged_out],
        )
        .mount(
            "/user",
            rocket_routes![routes::user::logged_out, routes::user::page],
        )
        .mount(
            "/posts",
            rocket_routes![
                routes::posts::logged_out_and_not_allowed,
                routes::posts::page,
            ],
        )
        .mount(
            "/posts/api",
            rocket_routes![
                routes::posts::api::community_posts::get,
                routes::posts::api::post_info::get
            ],
        )
        .mount("/build", FileServer::from("build"))
        .mount("/assets", FileServer::from("assets"));

    rocket
}

pub fn create_config(env: Env) -> Figment {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = dotenv::var("ROCKET_SECRET_KEY").expect("ROCKET_SECRET_KEY must be set");
    let service_account_path = dotenv::var("SERVICE_ACCOUNT").expect("SERVICE_ACCOUNT must be set");

    let mut db_config: Map<_, Value> = Map::new();
    let mut pg_config: Map<_, Value> = Map::new();

    let rate_limit_capacity: u32 = match env {
        Env::Development => 100,
        Env::Testing => 2,
        Env::Production => 100,
    };

    db_config.insert("url", db_url.into());
    pg_config.insert("sqlx", db_config.into());

    let figment = rocket::Config::figment()
        .merge(("databases", pg_config))
        .merge(("rate-limit-capacity", rate_limit_capacity))
        .merge(("secret_key", secret_key))
        .merge(("service_account", service_account_path));

    figment
}
