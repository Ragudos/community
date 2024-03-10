use crate::controllers::htmx::{redirect::HtmxRedirect, refresh::HtmxRefresh};
use rocket::{
    http::{ContentType, Status},
    response::Redirect,
};
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
}
