use crate::models::api::ApiResponse;
use rocket::{http::{Header, Status}, response::Responder, Response};
use std::io::Cursor;

impl<'a> Responder<'a, 'static> for ApiResponse {
    fn respond_to(self, request: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Self::HtmxRefresh(htmx_refresh) => htmx_refresh.respond_to(request),
            Self::HtmxRedirect(htmx_redirect) => htmx_redirect.respond_to(request),
            Self::Redirect(redirect) => redirect.respond_to(request),
            Self::String(status, string) => Response::build()
                .status(status)
                .streamed_body(Cursor::new(string))
                .ok(),
            Self::StringDynamic(status, string) => Response::build()
                .status(status)
                .streamed_body(Cursor::new(string))
                .ok(),
            Self::CustomHTML(status, ct, html) => Response::build()
                .status(status)
                .header(Header::new("Content-Type", ct.to_string()))
                .streamed_body(Cursor::new(html))
                .ok(),
            Self::Template(template) => template.respond_to(request),
            Self::NoContent => Response::build()
                .status(rocket::http::Status::NoContent)
                .ok(),
            Self::Created(resource_uri, html) => {
                let mut response = Response::build();

                response
                    .header(Header::new("Location", resource_uri))
                    .status(Status::Created);

                if let Some((content_type, html)) = html {
                    response
                        .header(Header::new("Content-Type", content_type.to_string()))
                        .streamed_body(Cursor::new(html))
                        .ok()
                } else {
                    response.ok()
                }
            }
        }
    }
}
