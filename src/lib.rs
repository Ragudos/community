use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::RwLock;

use controllers::rate_limiter::RateLimiter;
use models::notifications::RealtimeNotification;
use responders::ApiResponse;
use rocket;
use rocket::figment::value::{Map, Value};
use rocket::figment::{Figment, Profile, Provider};
use rocket::fs::FileServer;
use rocket::Build;
use rocket::Rocket;
use rocket::{catchers as rocket_catchers, routes as rocket_routes};
use rocket_csrf_token::{CsrfConfig, Fairing};
use time::OffsetDateTime;

use crate::catchers as main_catchers;
use crate::env::{Env, Environment};
use crate::helpers::db;
use crate::helpers::get_environment;
use crate::helpers::handlebars;
use crate::routes::auth::api::catchers as auth_api_catchers;
use crate::routes::auth::catchers as auth_catchers;
use crate::routes::community::catchers as community_catchers;
use crate::routes::notifications::catchers as notifications_catchers;

pub mod catchers;
pub mod controllers;
pub mod csp;
pub mod env;
pub mod helpers;
pub mod models;
pub mod responders;
pub mod routes;

#[rocket::get("/test")]
pub fn test_toast() -> ApiResponse {
    ApiResponse::Toast(
        rocket::http::Status::Ok,
        crate::models::Toast::success(Some("This is a test".to_string())),
    )
}

pub fn rocket_from_config(figment: Figment) -> Rocket<Build> {
    let rate_limiter = get_rate_limiter(&figment);
    let rocket = rocket::custom(figment)
        .manage(rate_limiter)
        .mount("/", rocket_routes![routes::page, test_toast]);
    let rocket = attach_env(rocket);
    let rocket = attach_fairings(rocket);
    let rocket = register_catchers(rocket);
    let rocket = attach_static_files(rocket);
    let rocket = attach_global_state(rocket);
    let rocket = attach_auth_routes(rocket);
    let rocket = attach_community_routes(rocket);
    let rocket = attach_discover_routes(rocket);
    let rocket = attach_create_routes(rocket);
    let rocket = attach_user_routes(rocket);
    let rocket = attach_post_routes(rocket);
    let rocket = attact_notification_routes(rocket);

    rocket
}

fn attact_notification_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/notifications",
        rocket_routes![
            routes::notifications::sse_notifications,
            routes::notifications::notifications
        ],
    )
}

fn attach_post_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount(
            "/posts",
            rocket_routes![routes::posts::logged_out, routes::posts::page,],
        )
        .mount(
            "/posts/api",
            rocket_routes![
                routes::posts::api::community_posts::get,
                routes::posts::api::malformed_uid,
                routes::posts::api::logged_out,
                routes::posts::api::post_info::get
            ],
        )
}

fn attach_user_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount(
            "/user",
            rocket_routes![routes::user::logged_out, routes::user::page],
        )
        .mount(
            "/user/api",
            rocket_routes![
                routes::user::api::img::user_img_endpoint
            ],
        )
}

fn attach_create_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount(
            "/create",
            rocket_routes![routes::create::community::community_page],
        )
        .mount(
            "/create/api",
            rocket_routes![
                routes::create::api::community::create_community_endpoint,
                routes::create::api::community::create_community_unauthorized
            ],
        )
}

fn attach_discover_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount(
            "/discover",
            rocket_routes![
                routes::discover::discover_page,
                routes::discover::unauthorized_discover
            ],
        )
        .mount(
            "/discover/api",
            rocket_routes![
                routes::discover::api::discover_endpoint,
                routes::discover::api::discover_endpoint_unauthorized
            ],
        )
}

fn attach_auth_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount(
            "/auth",
            rocket_routes![
                routes::auth::logged_in,
                routes::auth::logged_out,
                routes::auth::redirect_login,
                routes::auth::redirect_register,
                routes::auth::login::login_page,
                routes::auth::register::register_page
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
}

fn attach_community_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount(
            "/community",
            rocket_routes![
                routes::community::community_page,
                routes::community::about::about_community_page,
                routes::community::members::community_members_page,
                routes::community::settings::community_settings_page,
                routes::community::delete_community::delete_community_page,
                routes::community::change_join_process::change_join_process_page,
            ],
        )
        .mount(
            "/community/api",
            rocket_routes![
                routes::community::api::logged_out,
                routes::community::api::rename::rename_endpoint,
                routes::community::api::rename::rename_unauthorized,
                routes::community::api::rename::non_htmx_rename_endpoint,
                routes::community::api::rename::non_htmx_rename_unauthorized,
                routes::community::api::request_deletion::request_deletion_endpoint,
                routes::community::api::request_deletion::unauthorized_request_deletion,
                routes::community::api::request_change_join_process::request_change_join_process_endpoint,
                routes::community::api::request_change_join_process::unauthorized_request_change_join_process,
                routes::community::api::change_join_process::change_join_process_endpoint,
                routes::community::api::change_join_process::change_join_process_unauthorized_endpoint,
                routes::community::api::delete_community::delete_community_endpoint,
                routes::community::api::delete_community::unauthorized_delete_community,
                routes::community::api::leave_community::leave_community_endpoint,
                routes::community::api::leave_community::unauthorized_leave_community,
            ],
        )
        .mount(
            "/community/api/join",
            rocket_routes![
                routes::community::api::join::public::public_join_post,
                routes::community::api::join::public::unauthorized_join_public,
                routes::community::api::join::private::private_join_post,
                routes::community::api::join::private::unauthorized_join_private,
            ],
        )
}

fn attach_global_state(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.manage(rocket::tokio::sync::broadcast::channel::<RealtimeNotification>(1024).0)
}

fn attach_static_files(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/build", FileServer::from("build"))
        .mount("/assets", FileServer::from("assets"))
}

fn register_catchers(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .register("/", rocket_catchers![main_catchers::unauthorized_catcher])
        .register(
            "/community",
            rocket_catchers![
                community_catchers::community_page_not_found_get,
                community_catchers::community_page_unauthorized_get,
                community_catchers::community_page_internal_server_error_get,
                community_catchers::community_page_forbidden_get
            ],
        )
        .register(
            "/auth",
            rocket_catchers![
                auth_catchers::auth_internal_server_error_get,
                auth_catchers::auth_api_forbidden
            ],
        )
        .register(
            "/auth/api",
            rocket_catchers![
                auth_api_catchers::auth_api_internal_server_error,
                auth_api_catchers::forbidden_auth_api
            ],
        )
        .register(
            "/notifications",
            rocket_catchers![notifications_catchers::unauthorized_notifications],
        )
}

fn attach_fairings(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .attach(Fairing::new(
            CsrfConfig::default()
                .with_cookie_len(32)
                .with_cookie_name("CSRF-TOKEN"),
        ))
        .attach(db::stage())
        .attach(handlebars::register())
}

fn get_rate_limiter(figment: &Figment) -> RateLimiter {
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
        time_accumulator_started: RwLock::new(OffsetDateTime::now_utc()),
        did_time_accumulator_start: AtomicBool::new(false),
        requests: AtomicU32::new(0),
    };

    rate_limiter
}

fn attach_env(rocket: Rocket<Build>) -> Rocket<Build> {
    let environment = Environment {
        environment: get_environment().into(),
    };

    rocket.manage(environment)
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
