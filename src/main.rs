#[macro_use]
extern crate rocket;

use rocket as rocket_mod;

use community::{
    api,
    helpers::{db, handlebars},
};
use rocket_mod::{figment::Figment, fs::FileServer, Build, Config, Rocket};

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let config = Config::figment();
    rocket_from_config(config)
}

fn rocket_from_config(figment: Figment) -> Rocket<Build> {
    let rocket = rocket_mod::custom(figment)
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
                api::get::auth::root::page,
                api::get::auth::deny_welcome_page
            ],
        )
        .mount(
            "/homepage",
            routes![api::get::homepage::root::page, api::get::homepage::redirect],
        )
        .mount("/assets", FileServer::from("assets"))
        .attach(db::stage())
        .attach(handlebars::register());

    rocket
}

#[cfg(test)]
mod test_utils {
    use rocket::{
        figment::value::{Map, Value},
        local::asynchronous::Client,
    };

    use crate::rocket_from_config;

    pub async fn get_client() -> Client {
        let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut db_config: Map<_, Value> = Map::new();
        let mut pg_config: Map<_, Value> = Map::new();

        db_config.insert("url", db_url.into());
        pg_config.insert("sqlx", db_config.into());

        let figment = rocket::Config::figment().merge(("databases", pg_config));

        let client = Client::tracked(rocket_from_config(figment))
            .await
            .expect("valid rocket instance");

        client
    }
}

#[cfg(test)]
mod general_tests {
    use rocket::http::{ContentType, Status};

    use crate::test_utils::get_client;

    #[rocket::async_test]
    async fn test_connection() {
        let client = get_client().await;
        let response = client.get("/").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_register() {
        let client = get_client().await;
        let response = client.get("/auth/register").dispatch().await;

        assert_eq!(response.status(), Status::Ok);

        let mut request = client.post("/auth/register")
            .header(ContentType::Form)
            .body(r#"username=deadkiller&password.input=password&password.confirmation=password&gender=male&g-recaptcha-response="#);

        request.add_header(ContentType::Form);

        let response = request.dispatch().await;

        assert_eq!(response.status(), Status::Unauthorized);
        assert_eq!(
            response.into_string().await,
            Some("Please verify that you're not a robot.".to_string())
        );

        let mut request = client.post("/auth/register")
            .header(ContentType::Form)
            .body(r#"username=deadkiller&password.input=password&password.confirmation=password&gender=male&g-recaptcha-response=test"#);

        request.add_header(ContentType::Form);

        let response = request.dispatch().await;

        assert_eq!(response.status(), Status::Conflict);
        assert_eq!(
            Some("Please choose a different username.".to_string()),
            response.into_string().await
        );
    }
}
