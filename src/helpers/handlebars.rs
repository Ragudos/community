use rocket::fairing::Fairing;
use rocket_dyn_templates::{handlebars::handlebars_helper, Template};

handlebars_helper!(num_abbr: |x: isize| {
    if x < 1_000 {
        x.to_string()
    } else if x < 1_000_000 {
        format!("{:.1}k", x as f64 / 1_000.0)
    } else if x < 1_000_000_000 {
        format!("{:.1}m", x as f64 / 1_000_000.0)
    } else {
        format!("{:.1}b", x as f64 / 1_000_000_000.0)
    }
});
handlebars_helper!(add: |x: isize, y: isize| x + y);
handlebars_helper!(sub: |x: isize, y: isize| x - y);
handlebars_helper!(modulo: |x: isize, y: isize| x % y);

pub fn register() -> impl Fairing {
    Template::custom(|engines| {
        engines
            .handlebars
            .register_helper("num_abbr", Box::new(num_abbr));
        engines.handlebars.register_helper("add", Box::new(add));
        engines.handlebars.register_helper("sub", Box::new(sub));
        engines.handlebars.register_helper("modulo", Box::new(modulo));
    })
}
