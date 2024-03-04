use rocket::form::{Error, Result};

pub fn check_name<'v>(name: &'v str) -> Result<'v, ()> {
    if name.len() > 60 {
        Err(Error::validation(
            "Name can only contain up to 60 characters.",
        ))?;
    }

    if !name.chars().all(|c| c.is_whitespace() || c.is_alphanumeric()) {
        Err(Error::validation(
            "Name can only contain alphanumeric characters.",
        ))?;
    }

    Ok(())
}
