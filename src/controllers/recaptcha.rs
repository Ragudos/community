use rocket::http::Status;

use crate::models::{
    api::ApiResponse, captcha::RecaptchaResponse, errors::captcha::RecaptchaErrorCodes,
};

/// if and error occurs, return an API respone,
/// so the caller doesn't have to handle the error.
pub async fn verify_token(token: &str) -> Result<RecaptchaResponse, ApiResponse> {
    if token.is_empty() {
        return Err(ApiResponse::String(
            rocket::http::Status::Unauthorized,
            "Please verify that you're not a robot.",
        ));
    }

    let secret = dotenv::var("RECAPTCHA_SECRET_KEY")?;
    let client = reqwest::Client::new();
    let body = format!("secret={}&response={}", secret, token);
    let response = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    let text = response.text().await?;
    let payload = rocket::serde::json::from_str::<RecaptchaResponse>(&text)?;

    if payload.success == false {
        let Some(error_codes) = payload.error_codes else {
            return Err(ApiResponse::String(
                Status::Unauthorized,
                "Please verify that you are not a robot.",
            ));
        };

        if error_codes.contains(&RecaptchaErrorCodes::TimeoutOrDuplicate.into()) {
            return Err(ApiResponse::String(
                Status::TooManyRequests,
                "You have either taken too long to register after taking the reCaptcha, or you have already used your current token. Please retake the reCaptcha.",
            ));
        }

        if error_codes.contains(&RecaptchaErrorCodes::MissingResponse.into()) {
            return Err(ApiResponse::String(
                Status::Unauthorized,
                "Please verify that you are not a robot.",
            ));
        }

        if error_codes.contains(&RecaptchaErrorCodes::BadRequest.into()) {
            return Err(ApiResponse::String(
                Status::BadRequest,
                "Something went wrong. Please retake the reCaptcha.",
            ));
        }

        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Something went wrong.",
        ));
    }

    Ok(payload)
}
