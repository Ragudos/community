#[macro_export]
macro_rules! create_request_sensitive_action_jwt {
    ($name:ident, $path:literal) => {
        #[derive(rocket::serde::Serialize, rocket::serde::Deserialize, Debug)]
        pub struct $name {
            pub community_id: i64,
            pub user_id: i64,
            pub expires_in: time::OffsetDateTime,
        }

        impl $name {
            pub fn new(community_id: i64, user_id: i64) -> Self {
                Self {
                    community_id,
                    user_id,
                    expires_in: time::OffsetDateTime::now_utc().saturating_add(time::Duration::minutes(5)),
                }
            }

            pub fn to_cookie(&self) -> Result<rocket::http::Cookie<'static>, serde_json::Error> {
                let stringified = serde_json::to_string(self)?;

                Ok(rocket::http::Cookie::build((stringify!($name), stringified))
                    .same_site(rocket::http::SameSite::Strict)
                    .path($path)
                    .secure(true)
                    .http_only(true)
                    .same_site(rocket::http::SameSite::Strict)
                    .expires(self.expires_in)
                    .build())
            }
        }

        #[rocket::async_trait]
        impl<'a> rocket::request::FromRequest<'a> for $name {
            type Error = &'a str;

            async fn from_request(request: &'a rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
                let Some(cookie) = request.cookies().get_private(stringify!($name)) else {
                    return rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized);
                };
                let stringified_jwt = cookie.value();
                let Ok(jwt) = serde_json::from_str::<$name>(stringified_jwt) else {
                    // Means that the JWT has probably been tampered with.
                    request.cookies().remove_private(stringify!($name));
                    return rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized);
                };
                let user_jwt_outcome = request.guard::<crate::models::users::schema::UserJWT>().await;

                match user_jwt_outcome {
                    rocket::request::Outcome::Success(user_jwt) => {
                        if jwt.user_id == user_jwt._id {
                            if jwt.expires_in > time::OffsetDateTime::now_utc() {
                                rocket::request::Outcome::Success(jwt)
                            } else {
                                request.cookies().remove_private(stringify!($name));
                                rocket::request::Outcome::Error((
                                    rocket::http::Status::Unauthorized,
                                    "Your request has expired. Please submit another request for deletion.",
                                ))
                            }
                        } else {
                            rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized)
                        }
                    }
                    rocket::request::Outcome::Forward(status) => rocket::request::Outcome::Forward(status),
                    rocket::request::Outcome::Error((status, error)) => rocket::request::Outcome::Error((status, error)),
                }
            }
        }
    };
}
