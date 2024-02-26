#[macro_use]
extern crate rocket;

use rocket as rocket_mod;

use community::{
    api,
    helpers::{db, handlebars},
};
use rocket_mod::fs::FileServer;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let rocket = rocket_mod::build()
        .mount("/", routes![api::get::root::page,])
        .mount(
            "/auth",
            routes![
                api::post::auth::deny_post_request,
                api::post::auth::register::api_endpoint,
                api::post::auth::login::api_endpoint,
                api::delete::auth::logout::api_endpoint,
                api::delete::auth::logout::deny_delete_request,
                api::get::auth::redirect,
                api::get::auth::login::page,
                api::get::auth::register::page,
            ],
        )
        .mount("/assets", FileServer::from("assets"))
        .attach(db::stage())
        .attach(handlebars::register());

    rocket
}
