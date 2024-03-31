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
pub async fn discover_endpoint<'r>(
    mut db: Connection<DbConn>,
    _user: UserJWT,
    list_query: Option<ListQuery<'r>>,
) -> Result<ApiResponse, ApiResponse> {
    let offset = list_query
        .as_ref()
        .map(|list_query| list_query.offset.unwrap_or(0))
        .unwrap_or(0);
    let query: Option<&str> = list_query
        .as_ref()
        .map(|list_query| list_query.search)
        .unwrap_or(None);
    let mut vector = Vec::with_capacity(1);
    let active_categories = if let Some(list_query) = list_query {
        if let Some(category) = list_query.category {
            vector.push(category);

            Some(vector.as_slice())
        } else {
            None
        }
    } else {
        None
    };
    let communities = Community::get_by_weighted_score(
        &mut db,
        &offset,
        &HOMEPAGE_COMMUNITY_LIMIT,
        active_categories,
        query,
    )
    .await;
    let (communities, did_error) = match communities {
        Ok(communities) => (Some(communities), false),
        Err(e) => {
            eprintln!("Error getting communities: {:?}", e);
            (None, true)
        }
    };

    let pagination = if did_error {
        None
    } else {
        let pagination = Community::get_pagination(
            &mut db,
            &HOMEPAGE_COMMUNITY_LIMIT,
            active_categories,
            query,
        )
        .await;
        match pagination {
            Ok(pagination) => Some(
                pagination
                    .map_or(0, |p| p.to_string().parse::<i64>().unwrap_or(0)),
            ),
            Err(e) => {
                eprintln!("Error getting pagination: {:?}", e);
                None
            }
        }
    };

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/discover/api",
            context! { offset, communities, pagination, did_error },
        )),
        headers: None,
    })
}
