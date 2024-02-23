#[macro_use]
extern crate rocket;

use rocket as rocket_mod;

use community::helpers::{db, handlebars};

#[launch]
fn rocket() -> _ {
    rocket_mod::build()
        .attach(db::stage())
        .attach(handlebars::register())
}

