use rocket::http::{Header, Status};
use rocket::response::{Redirect, Responder, Response};
use rocket_csrf_token::VerificationFailure;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::refresh::HtmxRefresh;
use crate::models::Toast;

#[derive(Debug)]
pub enum HeaderCount {
    One(Header<'static>),
    Many(Vec<Header<'static>>),
}

#[derive(Debug)]
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
    Toast(Status, Toast),
}

impl From<VerificationFailure> for ApiResponse {
    fn from(_: VerificationFailure) -> Self {
        ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "partials/toast",
                context! {
                    toast: Toast::error(Some("You are no longer permitted to perform this action. Please try refreshing the page.".to_string()))
                },
            )),
            headers: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ToastTrigger {
    #[serde(rename = "HxEvent:Toast")]
    toast: Toast,
}

impl<'a> Responder<'a, 'static> for ApiResponse {
    fn respond_to(
        self,
        request: &'a rocket::Request<'_>,
    ) -> rocket::response::Result<'static> {
        match self {
            ApiResponse::Toast(status, toast) => {
                let trigger_header = ToastTrigger { toast };
                let response = Response::build()
                    .status(status)
                    .header(Header::new(
                        "HX-Trigger",
                        serde_json::to_string(&trigger_header).unwrap(),
                    ))
                    .header(Header::new("Hx-Reswap", "innerHTML"))
                    .ok();

                response
            }
            ApiResponse::Status(status) => status.respond_to(request),
            ApiResponse::HtmxRedirect(htmx_redirect) => {
                htmx_redirect.respond_to(request)
            }
            ApiResponse::HtmxRefresh(htmx_refresh) => {
                htmx_refresh.respond_to(request)
            }
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
