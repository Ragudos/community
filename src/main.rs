#[macro_use]
extern crate rocket;

use std::sync::{
    atomic::{AtomicBool, AtomicU32},
    RwLock,
};

use rocket::{
    self as rocket_mod,
    figment::{
        value::{Map, Value},
        Provider,
    },
};

use community::{
    api, catchers,
    helpers::{db, get_environment, handlebars},
    models::{rate_limiter::RateLimit, Env, Environment},
};
use rocket_mod::{
    figment::{Figment, Profile},
    fs::FileServer,
    Build, Rocket,
};

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let config = create_config(get_environment().into());
    rocket_from_config(config)
}

fn rocket_from_config(figment: Figment) -> Rocket<Build> {
    let rate_limit_capacity = figment.data().unwrap();
    let rate_limit_capacity = rate_limit_capacity.get(&Profile::Global).unwrap();
    let rate_limit_capacity = rate_limit_capacity
        .get("rate-limit-capacity")
        .unwrap()
        .to_num()
        .unwrap()
        .to_u32()
        .unwrap();

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
            ],
        )
        .mount(
            "/homepage",
            routes![api::get::homepage::root::page, api::get::homepage::redirect],
        )
        .mount(
            "/preview",
            routes![
                api::get::preview::community::api_endpoint,
                api::get::preview::user::api_endpoint,
                api::get::preview::deny_request
            ],
        )
        .mount(
            "/create",
            routes![
                api::get::create::community::page,
                api::get::create::redirect,
                api::post::create::deny_post_request,
                api::post::create::community::api_endpoint
            ],
        )
        .mount(
            "/community",
            routes![
                api::get::community::redirect,
                api::get::community::uid::about::page
            ],
        )
        .mount("/build", FileServer::from("build"))
        .mount("/assets", FileServer::from("assets"))
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
        );

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

#[cfg(test)]
mod test_utils {
    use rocket::local::asynchronous::Client;

    use crate::{create_config, rocket_from_config, Env};

    pub async fn get_client() -> Client {
        let figment = create_config(Env::Testing);

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

    #[rocket::async_test]
    async fn test_login() {
        let client = get_client().await;
        let response = client.get("/auth/login").dispatch().await;

        assert_eq!(response.status(), Status::Ok);

        let mut request = client
            .post("/auth/login")
            .header(ContentType::Form)
            .body(r#"username=deadkiller&password=12345678&g-recaptcha-response=test"#);

        request.add_header(ContentType::Form);

        let response = request.dispatch().await;
        let redirect_uri = response.headers().get("HX-Redirect").collect::<String>();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(redirect_uri, "/homepage");
    }

    #[rocket::async_test]
    async fn test_limiter() {
        let client = get_client().await;

        for _ in 0..2 {
            let mut request = client
                .post("/auth/login")
                .header(ContentType::Form)
                .body(r#"username=deadkiller&password=12345678&g-recaptcha-response=test"#);

            request.add_header(ContentType::Form);
            request.dispatch().await;

            let request = client.delete("/auth/logout");

            request.dispatch().await;
        }

        let mut request = client
            .post("/auth/login")
            .header(ContentType::Form)
            .body(r#"username=deadkiller&password=12345678&g-recaptcha-response=test"#);

        request.add_header(ContentType::Form);

        let response = request.dispatch().await;
        assert_eq!(response.status(), Status::TooManyRequests);
    }
}
