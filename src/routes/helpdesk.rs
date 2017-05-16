use rocket_contrib::Template;
use models::context::HelpDeskIndex;
use models::navigation::get_nav_list;

#[get("/")]
pub fn index() -> Template {
    let context = HelpDeskIndex {
        nav_links: get_nav_list(),
        current_page: "Home".to_string()
    };

    Template::render("helpdesk/index", &context)
}

#[post("/new", format = "multipart/form-data")]
pub fn post_new() -> &'static str {
    
}

#[get("/new")]
pub fn new() -> Template {
    
}