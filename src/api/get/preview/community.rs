use rocket::{get, http::Status};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    helpers::db::DbConn,
    models::{api::ApiResponse, community::schema::Community, users::metadata::JWT},
};

/// offset is how much the database should offset the results by.
#[get("/community?<q>&<o>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    jwt: JWT,
    o: Option<i64>,
    q: Option<&str>,
) -> Result<ApiResponse, ApiResponse> {
    let offset = match o {
        Some(offset) => offset,
        None => 0,
    };
    let offset_p = offset * 20;

    if offset.is_negative() {
        return Err(ApiResponse::String(Status::BadRequest, "Offset cannot be negative"));
    }

    let communities = match q {
        Some(q) => {
            if q.is_empty() {
                Community::get_all_by_offset_weighted(&mut db, &20, &offset_p).await?
            } else {
                Community::search_all_by_offset_weighted(&mut db, &20, &offset_p, &q).await?
            }
        }
        None => Community::get_all_by_offset_weighted(&mut db, &20, &offset_p).await?,
    };
    let page_count = Community::get_communities_count(&mut db, q).await?;

    Ok(ApiResponse::Template(Template::render(
        "partials/components/community/search-result",
        context! {
            communities,
            user: jwt.token,
            offset,
            search: q,
            page_count: match page_count.clone() {
                Some(page_count) => page_count.to_string().parse::<u64>().unwrap(),
                None => 0
            },
            bread_crumbs: match page_count {
                Some(page_count) => {
                    let mut vec = Vec::new();
                    for i in 0..page_count.to_string().parse::<u64>().unwrap() {
                        vec.push(i);
                    }

                    vec
                },
                None => vec![]
            }
        },
    )))
}

#[get("/community/amount-of-members?<community_display_name>")]
pub async fn amount_of_members(
    mut db: Connection<DbConn>,
    _jwt: JWT,
    community_display_name: &str,
) -> Result<ApiResponse, ApiResponse> {
    let amount =
        Community::get_total_members_count_by_display_name(&mut db, community_display_name).await?;

    Ok(ApiResponse::Template(Template::render(
        "partials/components/community/amount_of_members",
        context! {
            amount
        },
    )))
}
