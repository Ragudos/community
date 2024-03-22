use std::str::FromStr;

use rocket::form::{Error, Result};
use sqlx::types::Uuid;

pub fn validate_password<'v>(_password: &str) -> Result<'v, ()> {
    Ok(())
}

pub fn validate_uuid<'v>(uuid: &str) -> Result<'v, ()> {
    if Uuid::from_str(uuid).is_ok() {
        return Ok(());
    }

    Err(Error::validation("Invalid UUID"))?
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
    if !display_name
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace())
    {
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
