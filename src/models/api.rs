use rocket::response::Redirect;

use crate::controllers::htmx::{redirect::HtmxRedirect, refresh::HtmxRefresh};

pub enum ApiResponse {
    Redirect(Redirect),
    HtmxRedirect(HtmxRedirect),
    HtmxRefresh(HtmxRefresh)
}

