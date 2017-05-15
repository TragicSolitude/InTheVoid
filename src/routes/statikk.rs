use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/<file..>")]
pub fn any(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("statikk/").join(file)).ok()
}