#[macro_export]
macro_rules! auth_uri {
    ($($t:tt)*) => {
        rocket::uri!("/auth", $($t)*)
    }
}


