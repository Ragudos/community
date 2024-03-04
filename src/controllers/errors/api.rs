use std::error::Error;

use reqwest::StatusCode;
use rocket::response::Redirect;

use crate::models::api::ApiResponse;

impl From<cloud_storage::Error> for ApiResponse {
    fn from(value: cloud_storage::Error) -> Self {
        eprintln!("Cloud Storage Error: {}", value);

        ApiResponse::String(
            rocket::http::Status::InternalServerError,
            "Something went wrong.",
        )
    }
}

impl From<std::io::Error> for ApiResponse {
    fn from(value: std::io::Error) -> Self {
        eprintln!("IO Error: {}", value);

        ApiResponse::String(
            rocket::http::Status::InternalServerError,
            "Something went wrong.",
        )
    }
}

/// Convert a `dotenv::Error` into an `ApiResponse` error,
/// so we don't have to repeat ourselves in using match statements.
impl From<dotenv::Error> for ApiResponse {
    fn from(value: dotenv::Error) -> Self {
        eprintln!("Dotenv Error: {}", value);

        ApiResponse::String(
            rocket::http::Status::InternalServerError,
            "Misconfiguration: Please contact the site owner.",
        )
    }
}

/// Convert a `serde_json::Error` into an `ApiResponse` error,
/// so we don't have to repeat ourselves in using match statements.
impl From<serde_json::Error> for ApiResponse {
    fn from(value: serde_json::Error) -> Self {
        eprintln!("Serde Error: {}", value);

        ApiResponse::String(
            rocket::http::Status::BadRequest,
            "Invalid or malformed request.",
        )
    }
}

/// Convert a `sqlx::Error` into an `ApiResponse` error,
/// so we don't have to repeat ourselves in using match statements.
impl From<sqlx::Error> for ApiResponse {
    fn from(value: sqlx::Error) -> Self {
        eprintln!("Sqlx Error: {}", value.to_string());

        ApiResponse::String(
            rocket::http::Status::InternalServerError,
            "Something went wrong.",
        )
    }
}

impl From<bcrypt::BcryptError> for ApiResponse {
    fn from(value: bcrypt::BcryptError) -> Self {
        eprintln!("Bcrypt Error: {}", value.to_string());

        ApiResponse::String(
            rocket::http::Status::InternalServerError,
            "Something went wrong.",
        )
    }
}

/// Convert a `reqwest::Error` into an `ApiResponse` error,
/// so we don't have to repeat ourselves in using match statements.
impl From<reqwest::Error> for ApiResponse {
    fn from(err: reqwest::Error) -> Self {
        eprintln!("Reqwest Error.\n\t Source: {:?}", err.source());

        if err.is_redirect() {
            if let Some(url) = err.url() {
                return ApiResponse::Redirect(Redirect::to(url.to_string()));
            }
        }

        err.status().map_or(
            ApiResponse::String(
                rocket::http::Status::InternalServerError,
                "Something went wrong.",
            ),
            |status| {
                eprintln!("\t\tStatus: {:?}", status);
                match status {
                    StatusCode::BAD_GATEWAY => ApiResponse::String(
                        rocket::http::Status::BadGateway,
                        "The server received an invalid response.",
                    ),
                    StatusCode::SERVICE_UNAVAILABLE => ApiResponse::String(
                        rocket::http::Status::ServiceUnavailable,
                        "The service is currently unavailable. Please try again later.",
                    ),
                    StatusCode::GATEWAY_TIMEOUT => ApiResponse::String(
                        rocket::http::Status::GatewayTimeout,
                        "The server took too long to respond.",
                    ),
                    StatusCode::BAD_REQUEST => ApiResponse::String(
                        rocket::http::Status::BadRequest,
                        "Invalid or malformed request.",
                    ),
                    StatusCode::FORBIDDEN => ApiResponse::String(
                        rocket::http::Status::Forbidden,
                        "You do not have permission to access this resource.",
                    ),
                    StatusCode::UNAUTHORIZED => ApiResponse::String(
                        rocket::http::Status::Unauthorized,
                        "You are not authorized to access this resource.",
                    ),
                    StatusCode::NOT_FOUND => ApiResponse::String(
                        rocket::http::Status::NotFound,
                        "The resource you are looking for does not exist.",
                    ),
                    StatusCode::PRECONDITION_FAILED => ApiResponse::String(
                        rocket::http::Status::PreconditionFailed,
                        "The server does not meet one of the preconditions that the requester put on the request.",
                    ),
                    StatusCode::PAYLOAD_TOO_LARGE => ApiResponse::String(
                        rocket::http::Status::PayloadTooLarge,
                        "Request payload is too large. Please meet the requirements and try again.",
                    ),
                    StatusCode::NOT_IMPLEMENTED => ApiResponse::String(
                        rocket::http::Status::NotImplemented,
                        "The server does not support the functionality required to fulfill the request.",
                    ),
                    StatusCode::METHOD_NOT_ALLOWED => ApiResponse::String(
                        rocket::http::Status::MethodNotAllowed,
                        "Method not allowed.",
                    ),
                    StatusCode::CONFLICT => ApiResponse::String(
                        rocket::http::Status::Conflict,
                        "Request payload conflicts with the current state of the server.",
                    ),
                    StatusCode::UNPROCESSABLE_ENTITY => ApiResponse::String(
                        rocket::http::Status::UnprocessableEntity,
                        "Unable to process request payload.",
                    ),
                    StatusCode::TOO_MANY_REQUESTS => ApiResponse::String(
                        rocket::http::Status::TooManyRequests,
                        "Too many requests in a given amount of time.",
                    ),
                    StatusCode::REQUEST_TIMEOUT => ApiResponse::String(
                        rocket::http::Status::RequestTimeout,
                        "The server did not receive a complete request message within the time that it was prepared to wait.",
                    ),
                    StatusCode::LOOP_DETECTED => ApiResponse::String(
                        rocket::http::Status::LoopDetected,
                        "The server terminated an operation because it encountered an infinite loop while processing a request.",
                    ),
                    StatusCode::IM_A_TEAPOT => ApiResponse::String(
                        rocket::http::Status::ImATeapot,
                        "I'm a teapot.",
                    ),
                    StatusCode::INSUFFICIENT_STORAGE => ApiResponse::String(
                        rocket::http::Status::InsufficientStorage,
                        "Something went wrong.",
                    ),
                    StatusCode::HTTP_VERSION_NOT_SUPPORTED => ApiResponse::String(
                        rocket::http::Status::HttpVersionNotSupported,
                        "The server does not support the HTTP protocol version that was used in the request message.",
                    ),
                    StatusCode::EXPECTATION_FAILED => ApiResponse::String(
                        rocket::http::Status::ExpectationFailed,
                        "The server cannot meet the requirements of the Expect request-header field.",
                    ),
                    StatusCode::UPGRADE_REQUIRED => ApiResponse::String(
                        rocket::http::Status::UpgradeRequired,
                        "The server refuses to perform the request using the current protocol but might be willing to do so after the client upgrades to a different protocol.",
                    ),
                    StatusCode::MOVED_PERMANENTLY => ApiResponse::String(
                        rocket::http::Status::MovedPermanently,
                        "The resource has been moved permanently to a different location.",
                    ),
                    StatusCode::UNSUPPORTED_MEDIA_TYPE => ApiResponse::String(
                        rocket::http::Status::UnsupportedMediaType,
                        "The server is refusing to service the request because the entity of the request is in a format not supported by the requested resource for the requested method.",
                    ),
                    StatusCode::URI_TOO_LONG => ApiResponse::String(
                        rocket::http::Status::UriTooLong,
                        "The server is refusing to service the request because the request-target is longer than the server is willing to interpret.",
                    ),
                    _ => ApiResponse::String(
                        rocket::http::Status::InternalServerError,
                        "Something went wrong.",
                    ),
                }
            }
        )
    }
}
