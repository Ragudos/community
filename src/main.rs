use community::{create_config,rocket_from_config};
use community::helpers::get_environment;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let config = create_config(get_environment().into());
    rocket_from_config(config)
}
