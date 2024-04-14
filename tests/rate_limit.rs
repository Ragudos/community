mod utils;

use rocket::async_test;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};
use utils::get_client;

async fn fill_up_limiter<'r>(client: &'r Client) -> LocalResponse<'r> {
    for _ in 0..2 {
        let request = client
            .post("/auth/api/login")
            .header(ContentType::Form)
            .body(r#"username=deadkiller&password=12345678&honeypot=''"#);

        request.dispatch().await;

        let request = client.delete("/auth/api/logout");
        request.dispatch().await;
    }

    let request = client
        .post("/auth/api/login")
        .header(ContentType::Form)
        .body(r#"username=deadkiller&password=12345678&honeypot=''"#);

    request.dispatch().await
}

#[async_test]
#[ignore]
async fn rate_limiter_limits_successfully<'r>() {
    let client = get_client().await;
    let response = fill_up_limiter(&client).await;
    assert_eq!(response.status(), Status::TooManyRequests);
}
