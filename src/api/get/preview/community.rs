use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    helpers::db::DbConn,
    models::{
        api::ApiResponse, community::schema::Community, db::enums::CommunityCategory,
        users::schema::UserJWT, HOMEPAGE_COMMUNITY_LIMIT,
    },
};

/// offset is how much the database should offset the results by.
#[get("/community?<q>&<c>&<o>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    jwt: UserJWT,
    o: Option<i64>,
    c: Option<&str>,
    q: Option<&str>,
) -> Result<ApiResponse, ApiResponse> {
    let offset = match o {
        Some(offset) => offset,
        None => 0,
    };
    let c = match c {
        Some(c) => {
            if c.is_empty() {
                None
            } else {
                let split_values = c.split(',');
                Some(
                    split_values
                        .map(|s| s.into())
                        .take(3)
                        .collect::<Vec<CommunityCategory>>(),
                )
            }
        }
        None => None,
    };

    let (communities, page_count) = if let (Some(q), None) = (&q, &c) {
        let (communities, page_count) = if q.is_empty() {
            (
                Community::get_all_by_offset_and_weighted_score(
                    &mut db,
                    &offset,
                    &HOMEPAGE_COMMUNITY_LIMIT,
                )
                .await?,
                Community::get_pagination_count(&mut db, HOMEPAGE_COMMUNITY_LIMIT).await?,
            )
        } else {
            (
                Community::search_all_by_display_name_and_offset_and_weighted_score(
                    &mut db,
                    &offset,
                    &HOMEPAGE_COMMUNITY_LIMIT,
                    &q,
                )
                .await?,
                Community::get_pagination_count_filtered_by_display_name(
                    &mut db,
                    HOMEPAGE_COMMUNITY_LIMIT,
                    &q,
                )
                .await?,
            )
        };

        (communities, page_count)
    } else if let (None, Some(c)) = (&q, &c) {
        let communities = Community::search_all_by_category_and_offset_and_weighted_score(
            &mut db,
            &offset,
            &HOMEPAGE_COMMUNITY_LIMIT,
            &c,
        )
        .await?;
        let page_count =
            Community::get_pagination_filtered_by_category(&mut db, HOMEPAGE_COMMUNITY_LIMIT, &c)
                .await?;

        (communities, page_count)
    } else if let (Some(q), Some(c)) = (&q, &c) {
        let (communities, page_count) = if q.is_empty() {
            (
                Community::search_all_by_category_and_offset_and_weighted_score(
                    &mut db,
                    &offset,
                    &HOMEPAGE_COMMUNITY_LIMIT,
                    &c,
                )
                .await?,
                Community::get_pagination_filtered_by_category(
                    &mut db,
                    HOMEPAGE_COMMUNITY_LIMIT,
                    &c,
                )
                .await?,
            )
        } else {
            (
                Community::search_all_by_category_and_display_name_and_offset_and_weighted_score(
                    &mut db,
                    &offset,
                    &HOMEPAGE_COMMUNITY_LIMIT,
                    &c,
                    &q,
                )
                .await?,
                Community::get_pagination_filtered_by_category_and_display_name(
                    &mut db,
                    HOMEPAGE_COMMUNITY_LIMIT,
                    &c,
                    &q,
                )
                .await?,
            )
        };

        (communities, page_count)
    } else {
        let communities = Community::get_all_by_offset_and_weighted_score(
            &mut db,
            &offset,
            &HOMEPAGE_COMMUNITY_LIMIT,
        )
        .await?;
        let page_count = Community::get_pagination_count(&mut db, HOMEPAGE_COMMUNITY_LIMIT).await?;

        (communities, page_count)
    };

    println!("{:?}", communities);

    Ok(ApiResponse::Template(Template::render(
        "partials/components/community/search-result",
        context! {
            communities,
            user: jwt,
            offset,
            search: q,
            categories: c,
            page_count: match page_count.clone() {
                Some(page_count) => page_count.to_string().parse::<u64>().unwrap(),
                None => 0
            },
        },
    )))
}
