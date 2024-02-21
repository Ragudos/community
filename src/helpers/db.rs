use rocket::{
    fairing::{AdHoc, Result},
    Build, Rocket,
};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("sqlx")]
pub struct DbConn(sqlx::PgPool);

async fn run_migrations(rocket: Rocket<Build>) -> Result {
    match DbConn::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                eprintln!("Failed to run migrations: {:?}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

async fn seed_data(rocket: Rocket<Build>) -> Result {
    match DbConn::fetch(&rocket) {
        Some(db) => match sqlx::query_file!("db/sqlx/seed.sql").execute(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                eprintln!("Failed to seed data: {:?}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Warming up database...", |rocket| async {
        rocket
            .attach(DbConn::init())
            .attach(AdHoc::try_on_ignite(
                "Running database migrations...",
                run_migrations,
            ))
            .attach(AdHoc::try_on_ignite("Inserting seed data...", seed_data))
    })
}
