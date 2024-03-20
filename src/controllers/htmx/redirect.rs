use rocket::{
    http::{uri::Reference, Status},
    response::{Responder, Result},
    Response,
};
#[derive(Debug)]
pub struct HtmxRedirect(Status, Option<Reference<'static>>);

impl HtmxRedirect {
    pub fn to<U: TryInto<Reference<'static>>>(uri: U) -> HtmxRedirect {
        HtmxRedirect(Status::Ok, uri.try_into().ok())
    }
}

impl<'r> Responder<'r, 'static> for HtmxRedirect {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> Result<'static> {
        match self.1 {
            Some(uri) => Response::build()
                .status(self.0)
                .raw_header("HX-Redirect", uri.to_string())
                .ok(),
            None => {
                println!("Invalid URI for redirect.");
                Err(Status::InternalServerError)
            }
        }
    }
}
