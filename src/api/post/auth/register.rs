use rocket::post;
use rocket_db_pools::Connection;

use crate::{controllers::htmx::refresh::HtmxRefresh, helpers::db::DbConn, models::api::ApiResponse};


#[post("/register", data = "<register_data>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    register_data: 
) -> ApiResponse {
    ApiResponse::HtmxRefresh(HtmxRefresh)
}

