use rocket::http::{Header, Status};
use rocket::response::{Redirect, Responder, Response};
use rocket_dyn_templates::Template;

use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::refresh::HtmxRefresh;

pub enum HeaderCount {
    One(Header<'static>),
    Many(Vec<Header<'static>>),
}

pub enum ApiResponse {
    Status(Status),
    HtmxRedirect(HtmxRedirect),
    HtmxRefresh(HtmxRefresh),
    Redirect(Redirect),
    Render {
        status: Status,
        template: Option<Template>,
        headers: Option<HeaderCount>,
    },
}

impl<'a> Responder<'a, 'static> for ApiResponse {
    fn respond_to(self, request: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            ApiResponse::Status(status) => status.respond_to(request),
            ApiResponse::HtmxRedirect(htmx_redirect) => htmx_redirect.respond_to(request),
            ApiResponse::HtmxRefresh(htmx_refresh) => htmx_refresh.respond_to(request),
            ApiResponse::Redirect(redirect) => redirect.respond_to(request),
            ApiResponse::Render {
                status,
                template,
                headers,
            } => {
                let mut response = template.map_or_else(
                    || Ok(Response::build().ok()),
                    |template| Ok(template.respond_to(request)),
                )??;

                response.set_status(status);

                match headers {
                    Some(HeaderCount::One(header)) => {
                        response.set_header(header);
                    }
                    Some(HeaderCount::Many(headers)) => {
                        for header in headers {
                            response.set_header(header);
                        }
                    }
                    None => (),
                }

                Ok(response)
            }
        }
    }
}
