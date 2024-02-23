#[macro_use]
extern crate rocket;

use rocket as rocket_mod;

use community::{api, helpers::{db, handlebars}};

#[launch]
fn rocket() -> _ {
    rocket_mod::build()
        .mount(
            "/",
            routes![
                api::get::root::page,
            ]
        )
        .mount(
            "/auth",
            routes![
                api::post::auth::deny_post_request,
                api::post::auth::register::api_endpoint,
                api::get::auth::redirect,
                api::get::auth::login::page,
                api::get::auth::register::page
            ]
        )
        .attach(db::stage())
        .attach(handlebars::register())
}

