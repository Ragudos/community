use rocket::response::content::RawHtml;
use rocket::response::Responder;

#[derive(Responder)]
#[response(status = 401, content_type = "text/html")]
pub struct Unauthorized(RawHtml<&'static str>);
