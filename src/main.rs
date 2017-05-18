#![feature(plugin)]
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

fn main() {
    let path = Path::new(UPLOAD_DIR);
    if !path.exists() {
        fs::create_dir(path).unwrap();
    }
    routes::setup();
}