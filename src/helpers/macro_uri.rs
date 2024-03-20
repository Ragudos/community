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

#[macro_export]
macro_rules! community_uri {
    ($($t:tt)*) => {
        rocket::uri!("/community", $($t)*)
    }
}

#[macro_export]
macro_rules! discover_uri {
    ($($t:tt)*) => {
        rocket::uri!("/discover", $($t)*)
    }
}
