use rocket::{
    get,
    http::{CookieJar, Status},
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Metadata, Template};

use crate::{
    helpers::db::DbConn,
    models::{
        api::ApiResponse,
        community::schema::Community,
        seo::metadata::SeoMetadata,
        users::{preferences::Theme, schema::UserJWT},
        StringUuid,
    },
};

fn render_sqlx_error<'r>(metadata: &Metadata<'r>, jwt: &UserJWT, seo: SeoMetadata<'r>, error: sqlx::Error) -> ApiResponse {
    eprintln!("Sqlx Error {:?}", error);

    let (mime, html) = metadata.render("community/500", context! { user: jwt, metadata: seo }).unwrap();

    ApiResponse::CustomHTML(Status::InternalServerError, mime, html)
}

#[get("/<uid>/about")]
pub async fn page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    jwt: UserJWT,
    uid: StringUuid,
    metadata: Metadata<'r>,
) -> Result<ApiResponse, ApiResponse> {
    let StringUuid(community_uid) = uid;
    let theme = Theme::from_cookie_jar(&cookie_jar);
    let Some(community_info) = Community::get_by_uid(&mut db, &community_uid)
        .await
        .map_err(|error| {
            let seo = SeoMetadata::build().theme(theme.clone()).finalize();
            render_sqlx_error(&metadata, &jwt, seo, error)
        })?
    else {
        let seo = SeoMetadata::build().theme(theme).finalize();
        let (mime, html) = metadata
            .render(
                "community/404",
                context! { uid: community_uid.to_string(), user: &jwt, metadata: seo },
            )
            .unwrap();

        return Ok(ApiResponse::CustomHTML(Status::NotFound, mime, html));
    };
    let owner_uid_string = UserJWT::get_uid_by_id(&mut db, &community_info.owner_id)
        .await
        .map_err(|error| {
            let seo = SeoMetadata::build().theme(theme.clone()).finalize();
            render_sqlx_error(&metadata, &jwt, seo, error)
        })?;
    let cloned_name = community_info.display_name.clone();
    let seo = SeoMetadata::build()
        .theme(theme)
        .title(&cloned_name)
        .finalize();

    Ok(ApiResponse::Template(Template::render(
        "community/about/index",
        context! {
            metadata: seo,
            owner_uid: owner_uid_string,
            user: jwt,
            community: community_info,
        },
    )))
}
