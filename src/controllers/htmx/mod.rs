use std::convert::Infallible;

use rocket::{async_trait, request::{FromRequest, Outcome}, Request};

pub mod redirect;
pub mod refresh;

/// Whether a request to an endpoint is HTMX boosted
pub struct IsBoosted(pub bool);
pub struct IsHTMX(pub bool);

#[async_trait]
impl<'r> FromRequest<'r> for IsBoosted {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let is_boosted = request.headers().get_one("HX-Boosted");

        Outcome::Success(IsBoosted(match is_boosted {
            Some("true") => true,
            _ => false,
        }))
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for IsHTMX {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let is_htmx = request.headers().get_one("HX-Request");

        Outcome::Success(IsHTMX(match is_htmx {
            Some("true") => true,
            _ => false,
        }))
    }
}
