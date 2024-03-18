use community::models::Env;
use community::{create_config, rocket_from_config};
use rocket::local::asynchronous::Client;

pub async fn get_client() -> Client {
    let config = create_config(Env::Testing);

    Client::tracked(rocket_from_config(config))
        .await
        .expect("valid rocket instance")
}
