use rocket::data::Data;
use rocket_contrib::JSON;
use std::path::Path;
use std::io;
use std::fs::File;

use models::path::get_uuid;
use models::json::NewShareResponse;

use UPLOAD_DIR;

#[post("/", data = "<file>")]
pub fn index_file(file: Data) -> io::Result<JSON<NewShareResponse>> {
    let uuid: String = get_uuid();
    let path_str: &str = &format!("{}{}", UPLOAD_DIR, uuid);
    let mut file_handle: File = try!(File::create(Path::new(path_str)));
    let bytes_written: u64 = try!(io::copy(&mut file.open(), &mut file_handle));
    Ok(JSON(NewShareResponse { id: uuid, bytes: bytes_written }))
}

#[get("/<id>")]
pub fn index_id(id: &str) -> io::Result<File> {
    let path_str: &str = &format!("{}{}", UPLOAD_DIR, id);
    File::open(path_str)
}