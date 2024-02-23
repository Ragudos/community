use rocket::{form::Form, post};
use rocket_db_pools::Connection;

use crate::{controllers::htmx::refresh::HtmxRefresh, helpers::db::DbConn, models::{api::ApiResponse, forms::auth::RegisterFormData}};


#[post("/register", data = "<register_data>", rank = 2)]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    register_data: Form<RegisterFormData<'_>>,
) -> ApiResponse {
    ApiResponse::HtmxRefresh(HtmxRefresh)
}

