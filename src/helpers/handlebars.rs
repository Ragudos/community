use rocket::fairing::Fairing;
use rocket_dyn_templates::{handlebars::handlebars_helper, Template};

handlebars_helper!(eq_str: |x: str, y: str| x == y);
handlebars_helper!(eq_num: |x: isize, y: isize| x == y);
handlebars_helper!(is_str_empty: |x: str| x.is_empty());

pub fn register() -> impl Fairing {
    Template::custom(|engines| {
        engines
            .handlebars
            .register_helper("eq_str", Box::new(eq_str));
        engines
            .handlebars
            .register_helper("eq_num", Box::new(eq_num));
        engines
            .handlebars
            .register_helper("is_str_empty", Box::new(is_str_empty));
    })
}
