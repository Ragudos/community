#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::RwLock;

use rocket as rocket_mod;
use rocket::figment::value::{Map, Value};
use rocket::figment::{Figment, Profile, Provider};
use rocket::fs::FileServer;
use rocket::Build;
use rocket::Rocket;

use community::catchers;
use community::helpers::db;
use community::helpers::get_environment;
use community::helpers::handlebars;
use community::models::rate_limiter::RateLimit;
use community::models::{Env, Environment};
use community::routes;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let config = create_config(get_environment().into());
    rocket_from_config(config)
}

fn rocket_from_config(figment: Figment) -> Rocket<Build> {
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
    let rocket = rocket_mod::custom(figment)
        .attach(db::stage())
        .attach(handlebars::register())
        .manage(RateLimit {
            capacity: AtomicU32::new(rate_limit_capacity),
            time_accumulator_started: RwLock::new(time::OffsetDateTime::now_utc()),
            did_time_accumulator_start: AtomicBool::new(false),
            requests: AtomicU32::new(0),
        })
        .manage(Environment {
            environment: get_environment().into(),
        })
        .register(
            "/",
            catchers![
                catchers::unprocessable_entity,
                catchers::not_found,
                catchers::internal_server_error
            ],
        )
        .mount("/", routes![routes::page])
        .mount(
            "/auth",
            routes![
                routes::auth::logged_in,
                routes::auth::logged_out,
                routes::auth::login::page,
                routes::auth::register::page
            ],
        )
        .mount(
            "/auth/api",
            routes![
                routes::auth::api::login::post,
                routes::auth::api::register::post,
                routes::auth::api::logout::delete,
                routes::auth::api::login::logged_in,
                routes::auth::api::register::logged_in
            ],
        )
        .mount(
            "/community",
            routes![
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
            routes![
                routes::community::api::logged_out,
                routes::community::api::get,
                routes::community::api::uid::get
            ],
        )
        .mount(
            "/create",
            routes![routes::create::logged_out, routes::create::community::page],
        )
        .mount("/create/api", routes![routes::create::api::logged_out])
        .mount(
            "/user",
            routes![routes::user::logged_out, routes::user::page],
        )
        .mount(
            "/posts",
            routes![
                routes::posts::logged_out_and_not_allowed,
                routes::posts::page,
            ],
        )
        .mount(
            "/posts/api",
            routes![
                routes::posts::api::community_posts::get,
                routes::posts::api::post_info::get
            ]
        )
        .mount("/build", FileServer::from("build"))
        .mount("/assets", FileServer::from("assets"));

    rocket
}

fn create_config(env: Env) -> Figment {
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
