use crate::controllers::htmx::{redirect::HtmxRedirect, refresh::HtmxRefresh};
use rocket::http::Status;
use rocket::{http::ContentType, response::Redirect};
use rocket_dyn_templates::Template;

pub enum ApiResponse {
    Redirect(Redirect),
    HtmxRedirect(HtmxRedirect),
    HtmxRefresh(HtmxRefresh),
    String(Status, &'static str),
    StringDynamic(Status, String),
    CustomHTML(Status, ContentType, String),
    Template(Template),
    NoContent,
    /// This is used to return a 201 Created response with a location header.
    /// Also an optional html body can be provided.
    Created(String, Option<(ContentType, String)>),
}
