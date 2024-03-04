use rocket::{
    form::{Error, Result},
    fs::TempFile,
};

const ONE_MEGABYTE: u64 = 1_000_000;

pub fn validate_input<'v>(name: &str) -> Result<'v, ()> {
    if name.chars().any(|c| {
        if c == '<' || c == '>' {
            return true;
        }

        false
    }) {
        Err(Error::validation("String cannot contain '<' or '>'."))?;
    }

    Ok(())
}

pub fn check_image<'v>(file: &TempFile<'v>) -> Result<'v, ()> {
    if file.len() > ONE_MEGABYTE {
        Err(Error::validation("Image can only be up to 1MB."))?;
    }

    let Some(content_type) = file.content_type() else {
        return Err(Error::validation("Image must be a valid format."))?;
    };

    if content_type.is_avif()
        || content_type.is_webp()
        || content_type.is_jpeg()
        || content_type.is_png()
    {
        Ok(())
    } else {
        Err(Error::validation("Image must be a valid format."))?
    }
}
