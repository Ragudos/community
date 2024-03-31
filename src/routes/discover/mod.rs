use rocket::get;
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::auth_uri;
use crate::controllers::htmx::IsBoosted;
use crate::helpers::db::DbConn;
use crate::models::community::schema::Community;
use crate::models::query::ListQuery;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::models::HOMEPAGE_COMMUNITY_LIMIT;
use crate::responders::ApiResponse;
use crate::routes::auth::login;

pub mod api;

#[derive(Debug, Serialize)]
pub struct Category {
    name: &'static str,
    value: &'static str,
}

pub const CATEGORIES: [Category; 12] = [
    Category {
        name: "All",
        value: "",
    },
    Category {
        name: "🎨 Arts & Crafts",
        value: "art",
    },
    Category {
        name: "🎸 Music",
        value: "music",
    },
    Category {
        name: "🎮 Gaming",
        value: "gaming",
    },
    Category {
        name: "⚽ Sports",
        value: "sports",
    },
    Category {
        name: "⌛ Science",
        value: "science",
    },
    Category {
        name: "💻 Technology",
        value: "technology",
    },
    Category {
        name: "📚 Literature",
        value: "literature",
    },
    Category {
        name: "🍎 Health & Fitness",
        value: "healthandfitness",
    },
    Category {
        name: "📚 Self Improvement",
        value: "selfimprovement",
    },
    Category {
        name: "📚 Academics",
        value: "academics",
    },
    Category {
        name: "Other",
        value: "other",
    },
];

#[get("/?<isfromauth>&<list_query..>")]
pub async fn discover_page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    isfromauth: Option<bool>,
    list_query: Option<ListQuery<'r>>,
) -> Result<ApiResponse, ApiResponse> {
    let IsBoosted(is_boosted) = is_boosted;

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

    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("Discover Communities")
        .finalize();

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/discover",
            context! { categories: CATEGORIES, pagination, communities, did_error, metadata, user, is_boosted, isfromauth, offset, query, active_categories },
        )),
        headers: None,
    })
}

#[get("/", rank = 2)]
pub fn unauthorized_discover() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(Some(
        true
    )))))
}
