#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

use std::fs;
use std::path::Path;

// Path is relative to cwd
static UPLOAD_DIR: &str = "files/";

mod models;
mod routes;

// TODO Setup some kind of session-esque fuckery to
// ... send the little toats notifications to users
// ... from the server (eg. to indicate form submission
// ... status and whatnot
// -- Maybe we could set some kind of Response header
// --- that is read by the client
fn main() {
    let path = Path::new(UPLOAD_DIR);
    if !path.exists() {
        fs::create_dir(path).unwrap();
    }
    routes::setup();
}