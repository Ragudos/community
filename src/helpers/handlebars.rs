use rocket::fairing::Fairing;
use rocket_dyn_templates::{
    handlebars::{
        handlebars_helper, BlockContext, BlockParams, Context, Handlebars, Helper, Output,
        RenderContext, RenderError, Renderable,
    },
    Template,
};
use serde_json::Value;

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
handlebars_helper!(add: |x: i64, y: i64| x + y);
handlebars_helper!(sub: |x: i64, y: i64| x - y);
handlebars_helper!(modulo: |x: i64, y: i64| x % y);
handlebars_helper!(divide: |x: i64, y: i64| x / y);
handlebars_helper!(multiply: |x: i64, y: i64| x * y);
handlebars_helper!(max: |x: i64, y: i64| std::cmp::max(x, y));
handlebars_helper!(min: |x: i64, y: i64| std::cmp::min(x, y));

const MAX_BREAD_CRUMBS_IN_MIDDLE: i64 = 5;

pub fn breadcrumbs_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars<'reg>,
    c: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let offset: i64 = c.data().get("offset").and_then(|v| v.as_i64()).unwrap_or(0) + 1;
    let total_pages: i64 = c
        .data()
        .get("page_count")
        .and_then(|v| v.as_i64())
        .unwrap_or(1);

    if total_pages <= MAX_BREAD_CRUMBS_IN_MIDDLE {
        for i in 1..total_pages + 1 {
            let mut block_context = BlockContext::new();
            let mut block_param = BlockParams::new();

            block_param.add_value("value", Value::from(i))?;
            block_context.set_block_params(block_param);
            rc.push_block(block_context);

            let _ = h
                .template()
                .map(|t| t.render(r, c, rc, out))
                .unwrap_or(Ok(()));
            rc.pop_block();
        }
    } else {
        if offset <= MAX_BREAD_CRUMBS_IN_MIDDLE / 2 + 2 {
            for i in 1..MAX_BREAD_CRUMBS_IN_MIDDLE + 1 {
                let mut block_context = BlockContext::new();
                let mut block_param = BlockParams::new();
                block_param.add_value("value", Value::from(i))?;
                block_context.set_block_params(block_param);
                rc.push_block(block_context);
                let _ = h
                    .template()
                    .map(|t| t.render(r, c, rc, out))
                    .unwrap_or(Ok(()));
                rc.pop_block();
            }

            let _ = out.write("<span>...</span>");
            let mut block_context = BlockContext::new();
            let mut block_param = BlockParams::new();

            block_param.add_value("value", Value::from(total_pages))?;
            block_context.set_block_params(block_param);
            rc.push_block(block_context);

            let _ = h
                .template()
                .map(|t| t.render(r, c, rc, out))
                .unwrap_or(Ok(()));
            rc.pop_block();
        } else if offset >= total_pages - MAX_BREAD_CRUMBS_IN_MIDDLE / 2 - 1 {
            let mut block_context = BlockContext::new();
            let mut block_param = BlockParams::new();

            block_param.add_value("value", Value::from(1))?;
            block_context.set_block_params(block_param);
            rc.push_block(block_context);

            let _ = h
                .template()
                .map(|t| t.render(r, c, rc, out))
                .unwrap_or(Ok(()));
            rc.pop_block();
            let _ = out.write("<span>...</span>");

            for i in total_pages - MAX_BREAD_CRUMBS_IN_MIDDLE + 1..total_pages +  1 {
                let mut block_context = BlockContext::new();
                let mut block_param = BlockParams::new();
                block_param.add_value("value", Value::from(i))?;
                block_context.set_block_params(block_param);
                rc.push_block(block_context);
                let _ = h
                    .template()
                    .map(|t| t.render(r, c, rc, out))
                    .unwrap_or(Ok(()));
                rc.pop_block();
            }
        } else {
            let mut block_context = BlockContext::new();
            let mut block_param = BlockParams::new();

            block_param.add_value("value", Value::from(1))?;
            block_context.set_block_params(block_param);
            rc.push_block(block_context);

            let _ = h
                .template()
                .map(|t| t.render(r, c, rc, out))
                .unwrap_or(Ok(()));
            rc.pop_block();

            let _ = out.write("<span>...</span>");
            let start = offset - 1;
            let end = offset + 1;

            for i in start..end + 1 {
                let mut block_context = BlockContext::new();
                let mut block_param = BlockParams::new();
                block_param.add_value("value", Value::from(i))?;
                block_context.set_block_params(block_param);
                rc.push_block(block_context);
                let _ = h
                    .template()
                    .map(|t| t.render(r, c, rc, out))
                    .unwrap_or(Ok(()));
                rc.pop_block();
            }

            let _ = out.write("<span>...</span>");
            let mut block_context = BlockContext::new();
            let mut block_param = BlockParams::new();
            block_param.add_value("value", Value::from(total_pages))?;
            block_context.set_block_params(block_param);
            rc.push_block(block_context);
            let _ = h
                .template()
                .map(|t| t.render(r, c, rc, out))
                .unwrap_or(Ok(()));
            rc.pop_block()
        }
    }

    Ok(())
}

pub fn register() -> impl Fairing {
    Template::custom(|engines| {
        engines
            .handlebars
            .register_helper("num_abbr", Box::new(num_abbr));
        engines.handlebars.register_helper("add", Box::new(add));
        engines.handlebars.register_helper("sub", Box::new(sub));
        engines
            .handlebars
            .register_helper("modulo", Box::new(modulo));
        engines
            .handlebars
            .register_helper("divide", Box::new(divide));
        engines
            .handlebars
            .register_helper("multiply", Box::new(multiply));
        engines.handlebars.register_helper("max", Box::new(max));
        engines.handlebars.register_helper("min", Box::new(min));
        engines
            .handlebars
            .register_helper("breadcrumbs", Box::new(breadcrumbs_helper));
    })
}
