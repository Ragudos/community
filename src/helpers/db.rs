use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    fairing::{AdHoc, Result},
    Build, Rocket,
};
use rocket_db_pools::Database;
use sqlx::query;

use crate::models::users::metadata::{Gender, Occupation, UserRole};

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
        Some(db) => {
            let transaction = db.begin().await;

            match transaction {
                Ok(mut tx) => {
                    if let Err(e) = query!(
                        r#"
                            INSERT INTO users (id, display_name, display_image)
                            SELECT $1, $2, $3
                            WHERE NOT EXISTS (
                                SELECT 1 FROM users WHERE id = $1
                            );
                        "#,
                        0,
                        "deadkiller",
                        "/assets/dummy/user_display_image.jpg"
                    )
                    .execute(&mut *tx)
                    .await
                    {
                        eprintln!("Failed to seed data: {:?}", e);
                        let _ = tx.rollback().await;
                        return Err(rocket);
                    }

                    if let Err(e) = query!(
                        r#"
                            INSERT INTO users_metadata
                            (id, occupation, gender)
                            SELECT $1, $2, $3
                            WHERE NOT EXISTS (
                                SELECT 1 FROM users_metadata WHERE id = $1
                            );
                        "#,
                        0,
                        Occupation::Student as Occupation,
                        Gender::Male as Gender,
                    )
                    .execute(&mut *tx)
                    .await
                    {
                        eprintln!("Failed to seed data: {:?}", e);
                        let _ = tx.rollback().await;
                        return Err(rocket);
                    }

                    let hashed_password_result = hash("12345678", DEFAULT_COST);

                    match hashed_password_result {
                        Ok(hashed_password) => {
                            if let Err(e) = query!(
                                r#"
                                    INSERT INTO users_credentials
                                    (id, email, password_hash, first_name, last_name)
                                    SELECT $1, $2, $3, $4, $5
                                    WHERE NOT EXISTS (
                                        SELECT 1 FROM users_credentials WHERE id = $1
                                    );
                                "#,
                                0,
                                "johndoe@example.com",
                                hashed_password,
                                "",
                                "",
                            )
                            .execute(&mut *tx)
                            .await
                            {
                                eprintln!("Failed to seed data: {:?}", e);
                                let _ = tx.rollback().await;
                                return Err(rocket);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to hash password: {:?}", err);
                            let _ = tx.rollback().await;
                            return Err(rocket);
                        }
                    }

                    if let Err(e) = query!(
                        r#"
                            INSERT INTO communities
                            (id, display_name, display_image, description, is_private, owner_id, cover_image)
                            SELECT $1, $2, $3, $4, $5, $6, $7
                            WHERE NOT EXISTS (
                                SELECT 1 FROM communities WHERE id = $1 AND owner_id = $6
                            );
                        "#,
                        0,
                        "Rustaceans",
                        "/assets/dummy/community_display_image.jpg",
                        "A community for Rust developers",
                        false,
                        0,
                        "/assets/dummy/community_cover_image.jpg"
                    )
                    .execute(&mut *tx)
                    .await
                    {
                        eprintln!("Failed to seed data: {:?}", e);
                        let _ = tx.rollback().await;
                        return Err(rocket);
                    }

                    if let Err(e) = query!(
                        r#"
                            INSERT INTO community_memberships
                            (user_id, community_id, role)
                            SELECT $1, $2, $3 
                            WHERE NOT EXISTS (
                                SELECT 1 FROM community_memberships WHERE user_id = $1 AND community_id = $2
                            );
                        "#,
                        0,
                        0,
                        UserRole::Owner as UserRole
                    ).execute(&mut *tx).await {
                        eprintln!("Failed to seed data: {:?}", e);
                        let _ = tx.rollback().await;
                        return Err(rocket);
                    }

                    if let Err(e) = tx.commit().await {
                        eprintln!("Failed to commit transaction: {:?}", e);
                        return Err(rocket);
                    }

                    Ok(rocket)
                }
                Err(e) => {
                    eprintln!("Failed to start transaction: {:?}", e);
                    return Err(rocket);
                }
            }
        }
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
