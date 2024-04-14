use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::response::Responder;
use rocket::{get, Response};
pub struct CachedFile(pub NamedFile);

impl<'r> Responder<'r, 'static> for CachedFile {
    fn respond_to(
        self,
        request: &'r rocket::Request<'_>,
    ) -> rocket::response::Result<'static> {
        Response::build_from(self.0.respond_to(request)?)
            .raw_header("Cache-Control", "max-age=86400")
            .ok()
    }
}

#[get("/assets/<file..>")]
pub async fn asset_files(file: PathBuf) -> Option<CachedFile> {
    NamedFile::open(Path::new("assets/").join(file))
        .await
        .ok()
        .map(|nf| CachedFile(nf))
}

#[get("/build/<file..>")]
pub async fn build_files(file: PathBuf) -> Option<CachedFile> {
    NamedFile::open(Path::new("build/").join(file))
        .await
        .ok()
        .map(|nf| CachedFile(nf))
}
