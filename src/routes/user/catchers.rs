use rocket::catch;
use rocket::response::Redirect;

use crate::auth_uri;
use crate::routes::auth::login;

#[catch(401)]
pub fn page_unauthorized() -> Redirect {
    Redirect::to(auth_uri!(login::login_page(_)))
}
