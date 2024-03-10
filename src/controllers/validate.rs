use rocket::form::{Error, Result};

pub fn validate_password<'v>(password: &str) -> Result<'v, ()> {
    if password.len() < 8 {
        return Err(Error::validation(
            "Password must contain at least 8 characters.",
        ))?;
    }

    // Todo: Add more password validation here. Use Regex

    Ok(())
}

pub fn validate_password_with_confirmation<'v>(
    password: &'v str,
    confirm_password: &str,
) -> Result<'v, ()> {
    validate_password(password)?; // Validate password first

    if password != confirm_password {
        return Err(Error::validation("Passwords do not match"))?;
    }

    Ok(())
}

pub fn validate_display_name<'v>(display_name: &str) -> Result<'v, ()> {
    if display_name.len() > 60 {
        return Err(Error::validation(
            "Name can only contain up to 60 characters.",
        ))?;
    }

    if !display_name.chars().all(|c| c.is_alphanumeric()) {
        return Err(Error::validation(
            "Name can only contain alphanumeric characters.",
        ))?;
    }

    Ok(())
}

pub fn validate_honeypot<'v>(honeypot: &str) -> Result<'v, ()> {
    if !honeypot.is_empty() {
        return Err(Error::validation(
            "You were detected to be a bot. Please try again.",
        ))?;
    }

    Ok(())
}
