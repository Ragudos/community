#[macro_export]
macro_rules! auth_uri {
    ($($t:tt)*) => {
        rocket::uri!("/auth", $($t)*)
    }
}

#[macro_export]
macro_rules! homepage_uri {
    ($($t:tt)*) => {
        rocket::uri!("/homepage", $($t)*)
    }
}
