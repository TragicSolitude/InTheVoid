pub mod index;
pub mod share;

use rocket::Rocket;

pub fn setup() {
    Rocket::ignite()
        .mount("/", routes![index::index])
        .mount("/share/", routes![share::index_id])
        .mount("/share/", routes![share::index_file])
        .launch();
}