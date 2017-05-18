pub mod index;
pub mod share;
pub mod helpdesk;
pub mod statikk;

use rocket::Rocket;

pub fn setup() {
    Rocket::ignite()
        .mount("/", routes![index::index])
        .mount("/static/", routes![statikk::any])
        .mount("/share/", routes![share::index_id, share::index_file])
        .mount("/helpdesk", routes![helpdesk::index, helpdesk::post_new, helpdesk::new])
        .launch();
}