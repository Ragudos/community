use rocket::get;
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::community::schema::Community;
use crate::models::query::ListQuery;
use crate::models::users::schema::UserJWT;
use crate::models::HOMEPAGE_COMMUNITY_LIMIT;
use crate::responders::ApiResponse;

/// For displaying possible communities to join
#[get("/?<list_query..>")]
pub async fn get<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    list_query: Option<ListQuery<'r>>,
) -> Result<ApiResponse, ApiResponse> {
    // This cannot be None
    let offset = list_query
        .as_ref()
        .map(|list_query| list_query.offset.unwrap_or(0))
        .unwrap_or(0);
    let query: Option<&str> = list_query
        .as_ref()
        .map(|list_query| list_query.search)
        .unwrap_or(None);
    let categories = list_query
        .as_ref()
        .map(|list_query| list_query.category.as_ref())
        .unwrap_or(None);

    let communities = if let (Some(query), Some(categories)) = (query, categories) {
        Community::search_all_by_category_and_display_name_and_offset_and_weighted_score(
            &mut db,
            &offset,
            &HOMEPAGE_COMMUNITY_LIMIT,
            categories,
            query,
        )
        .await?
    } else if let (Some(query), None) = (query, categories) {
        Community::search_all_by_display_name_and_offset_and_weighted_score(
            &mut db,
            &offset,
            &HOMEPAGE_COMMUNITY_LIMIT,
            query,
        )
        .await?
    } else if let (None, Some(categories)) = (query, categories) {
        Community::search_all_by_category_and_offset_and_weighted_score(
            &mut db,
            &offset,
            &HOMEPAGE_COMMUNITY_LIMIT,
            categories,
        )
        .await?
    } else {
        Community::get_all_by_offset_and_weighted_score(&mut db, &offset, &HOMEPAGE_COMMUNITY_LIMIT)
            .await?
    };

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/discover",
            context! {
                user,
                communities,
                offset,
                categories,
                query
            },
        )),
        headers: None,
    })
}

/// Just a no content for any request made where the first
/// endpoint has forwarded.
#[get("/<_..>", rank = 2)]
pub fn logged_out() -> ApiResponse {
    ApiResponse::Status(Status::NoContent)
}
