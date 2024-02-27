use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    helpers::db::DbConn,
    models::{api::ApiResponse, community::schema::Community, users::metadata::JWT},
};

/// offset is how much the database should offset the results by.
#[get("/community?<offset>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    jwt: JWT,
    offset: Option<i64>,
) -> Result<ApiResponse, ApiResponse> {
    let offset = match offset {
        Some(offset) => (offset - 1) * 20,
        None => 0,
    };
    let communities = Community::get_all_by_offset_weighted(&mut db, &20, &offset).await?;

    Ok(ApiResponse::Template(Template::render(
        "partials/components/community/community_list",
        context! {
            communities,
            user: jwt.token
        },
    )))
}

#[get("/community/amount-of-members?<community_id>")]
pub async fn amount_of_members(
    mut db: Connection<DbConn>,
    _jwt: JWT,
    community_id: i32,
) -> Result<ApiResponse, ApiResponse> {
    let amount = Community::get_total_members_count(&mut db, community_id).await?;

    Ok(ApiResponse::Template(Template::render(
        "partials/components/community/amount_of_members",
        context! {
            amount
        },
    )))
}
