use rocket_contrib::Template;
use models::context::{HelpDeskIndex, HelpDeskNew};
use models::navigation::get_nav_list;

#[get("/")]
pub fn index() -> Template {
    let context = HelpDeskIndex {
        nav_links: get_nav_list(),
        current_page: "Home".to_string()
    };

    Template::render("helpdesk/index", &context)
}

#[get("/new")]
pub fn new() -> Template {
    let context = HelpDeskNew {
        nav_links: get_nav_list(),
        current_page: "New".to_string(),
        layout: "layout.html".to_string()
    };

    Template::render("helpdesk/new", &context)
}

#[post("/new", format = "multipart/form-data")]
pub fn post_new() -> &'static str {
    "{\"Test\": \"Successful\"}"
}