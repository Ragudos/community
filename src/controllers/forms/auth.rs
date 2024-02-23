use rocket::form::{Error, Result};

pub fn check_password<'v>(
    password: &'v str,
    confirm_password: &'v str
) -> Result<'v, ()> {
    if password.len() < 8 {
        Err(Error::validation("Password must contain at least 8 characters."))?;
    }

    if password.len() != confirm_password.len() {
        Err(Error::validation("Passwords do not match"))?;
    }

    Ok(())
}

/// A shallow check. Please check if the user is taken
/// on the api endpoint itself since idk how to connect to db on
/// this validator.
pub fn check_name<'v>(
    name: &'v str
) -> Result<'v, ()> {
    if name.len() > 60 {
        Err(Error::validation("Name can only contain up to 60 characters."))?;
    }

    Ok(()) 
}

