use rocket::{
    http::{Cookie, SameSite, Status},
    request::{FromRequest, Outcome},
    Request,
};
use time::{Duration, OffsetDateTime};

use crate::models::{
    community::request_deletion::RequestDeletionJWT, users::schema::UserJWT,
    REQUEST_DELETION_JWT_NAME,
};

impl RequestDeletionJWT {
    pub fn new(community_id: i64, user_id: i64) -> Self {
        Self {
            community_id,
            user_id,
            expires_in: OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)),
        }
    }

    pub fn to_cookie(&self) -> Result<Cookie<'static>, serde_json::Error> {
        let stringified = serde_json::to_string(self)?;

        Ok(Cookie::build((REQUEST_DELETION_JWT_NAME, stringified))
            .same_site(SameSite::Strict)
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .expires(self.expires_in)
            .build())
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for RequestDeletionJWT {
    type Error = &'a str;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(cookie) = request.cookies().get_private(REQUEST_DELETION_JWT_NAME) else {
            return Outcome::Forward(Status::Unauthorized);
        };
        let stringified_jwt = cookie.value();
        let Ok(jwt) = serde_json::from_str::<RequestDeletionJWT>(stringified_jwt) else {
            // Means that the JWT has probably been tampered with.
            request.cookies().remove_private(REQUEST_DELETION_JWT_NAME);
            return Outcome::Forward(Status::Unauthorized);
        };
        let user_jwt_outcome = request.guard::<UserJWT>().await;

        match user_jwt_outcome {
            Outcome::Success(user_jwt) => {
                if jwt.user_id == user_jwt._id {
                    if jwt.expires_in > OffsetDateTime::now_utc() {
                        Outcome::Success(jwt)
                    } else {
                        request.cookies().remove_private(REQUEST_DELETION_JWT_NAME);
                        Outcome::Error((
                            Status::Unauthorized,
                            "Your request has expired. Please submit another request for deletion.",
                        ))
                    }
                } else {
                    Outcome::Forward(Status::Unauthorized)
                }
            }
            Outcome::Forward(status) => Outcome::Forward(status),
            Outcome::Error((status, error)) => Outcome::Error((status, error)),
        }
    }
}
