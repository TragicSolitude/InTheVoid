use rocket_contrib::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use models::context::{HelpDeskIndex, HelpDeskNew};
use models::navigation::get_nav_list;
use models::form::NewIssueForm;

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

#[post("/new", data="<issue>")]
pub fn post_new(issue: Form<NewIssueForm>) -> Redirect {
    Redirect::to("/helpdesk")
}