use rocket::form::{Error, Result};

pub fn validate_password<'v>(_password: &str) -> Result<'v, ()> {
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

pub fn validate_ascii_text<'v>(display_name: &str) -> Result<'v, ()> {
    if !display_name.chars().all(|c| c.is_alphanumeric()) {
        return Err(Error::validation("No special characters are allowed."))?;
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
