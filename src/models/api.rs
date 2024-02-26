use std::io::Cursor;

use rocket::{
    http::Status,
    response::{Redirect, Responder},
    Response,
};
use rocket_dyn_templates::Template;

use crate::controllers::htmx::{redirect::HtmxRedirect, refresh::HtmxRefresh};

pub enum ApiResponse {
    Redirect(Redirect),
    HtmxRedirect(HtmxRedirect),
    HtmxRefresh(HtmxRefresh),
    String(Status, &'static str),
    StringDynamic(Status, String),
    Template(Template),
}

impl<'a> Responder<'a, 'static> for ApiResponse {
    fn respond_to(self, request: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Self::HtmxRefresh(htmx_refresh) => htmx_refresh.respond_to(request),
            Self::HtmxRedirect(htmx_redirect) => htmx_redirect.respond_to(request),
            Self::Redirect(redirect) => redirect.respond_to(request),
            Self::String(status, string) => Response::build()
                .status(status)
                .sized_body(string.len(), Cursor::new(string))
                .ok(),
            Self::StringDynamic(status, string) => Response::build()
                .status(status)
                .sized_body(string.len(), Cursor::new(string))
                .ok(),
            Self::Template(template) => template.respond_to(request),
        }
    }
}
