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

#[test]
fn test_helpers() {
    use rocket_dyn_templates::handlebars::Handlebars;

    let mut hbs = Handlebars::new();

    hbs.register_helper("eq_str", Box::new(eq_str));
    hbs.register_helper("eq_num", Box::new(eq_num));
    hbs.register_helper("is_str_empty", Box::new(is_str_empty));

    let eq_str_result = hbs.render_template(r#"{{#if (eq_str "a" "a")}}true{{else}}false{{/if}}"#, &());
    let eq_num_result = hbs.render_template(r#"{{#if (eq_num 1 1)}}true{{else}}false{{/if}}"#, &());
    let is_str_empty_result = hbs.render_template(r#"{{#if (is_str_empty "")}}true{{else}}false{{/if}}"#, &());

    assert_eq!(eq_str_result.unwrap(), "true");
    assert_eq!(eq_num_result.unwrap(), "true");
    assert_eq!(is_str_empty_result.unwrap(), "true");
}

