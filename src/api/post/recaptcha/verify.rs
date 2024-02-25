use rocket::{http::Status, post, serde::json::Json};
use reqwest::{self, StatusCode};
use crate::models::{api::ApiResponse, captcha::CaptchaToken};

#[post("/verify", format = "json", data = "<recaptcha_response>")]
pub async fn api_endpoint(
    recaptcha_response: Json<CaptchaToken<'_>>,
) -> ApiResponse {
    let secret = dotenv::var("RECAPTCHA_SECRET_KEY");

    match secret {
        Ok(secret) => {
            let client = reqwest::Client::new();
            let body = format!("secret={}&response={}", secret, recaptcha_response.token);
            let response = client.post("https://www.google.com/recaptcha/api/siteverify")
                .body(body)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .send()
                .await;

            match response {
                Ok(response) => {
                    match response.status() {
                        StatusCode::OK => {
                            let text = response.text().await;

                            match text {
                                Ok(text) => {
                                    ApiResponse::StringDynamic(Status::Ok, text)
                                },
                                Err(err) => {
                                    eprintln!("Error verifying recaptcha: {:?}", err);
                                    ApiResponse::String(Status::InternalServerError, "Unable to verify.")
                                }
                            }
                        },
                        _ => {
                            eprintln!("Error verifying recaptcha: {:?}", response);
                            return ApiResponse::String(Status::InternalServerError, "Unable to verify.");
                        }
                    }
                },
                Err(err) => {
                    eprintln!("Error verifying recaptcha: {:?}", err);
                    return ApiResponse::String(Status::InternalServerError, "Unable to verify.");
                }
            }
        },
        Err(err) => {
            eprintln!("Error getting secret: {:?}", err);
            ApiResponse::String(Status::InternalServerError, "Unable to verify.")
        }
    }
}
