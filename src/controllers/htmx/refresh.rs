use rocket::{
    http::Status,
    response::{Responder, Result},
    Response,
};

#[derive(Debug)]
pub struct HtmxRefresh;

impl<'a> Responder<'a, 'static> for HtmxRefresh {
    fn respond_to(self, _request: &'a rocket::Request<'_>) -> Result<'static> {
        Response::build()
            .raw_header("HX-Refresh", "true")
            .status(Status::ResetContent)
            .ok()
    }
}
