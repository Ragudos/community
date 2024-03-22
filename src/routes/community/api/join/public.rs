use std::str::FromStr;

use rocket::form::{Errors, Form};
use rocket::post;
use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::controllers::errors::extract_data_or_return_response;
use crate::controllers::htmx::IsHTMX;
use crate::models::community::schema::Community;
use crate::responders::ApiResponse;
use crate::helpers::db::DbConn;
use crate::models::users::schema::UserJWT;
use crate::models::StringUuidForm;

#[post("/join", data = "<form>")]
pub async fn post<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Result<Form<StringUuidForm>, Errors<'r>>,
    is_htmx: IsHTMX
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(form, "join_error")?;
    let community_uid = Uuid::from_str(&form.community_uid)?;

    if Community::is_private(&mut db, &community_uid).await? {
        unimplemented!()
    }

    unimplemented!()
}