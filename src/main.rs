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

    rocket_mod::build()
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
        .mount(
            "/recaptcha",
            routes! [
                api::post::recaptcha::verify::api_endpoint
            ]
        )
        .mount("/assets", FileServer::from("assets"))
        .attach(db::stage())
        .attach(handlebars::register())
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::{http::Status, local::asynchronous::Client};

    #[rocket::async_test]
    async fn test_db_user() {
        let client = Client::tracked(rocket()).await.unwrap();
    }
}

