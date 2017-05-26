use rocket_contrib::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use models::context::{HelpDeskIndex, HelpDeskNew};
use models::navigation::get_nav_list;
use models::form::NewIssueForm;
use models::db::{init, issues, Issue, NewIssue};
use diesel::*;

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

#[post("/new", data="<issue_form>")]
pub fn post_new(issue_form: Form<NewIssueForm>) -> Redirect {
    let connection: sqlite::SqliteConnection = init();

    let issue = issue_form.get();
    let new_issue = NewIssue {
        summary: issue.summary.to_string(),
        description: issue.description.to_string(),
        priority: 3
    };

    insert(&new_issue)
        .into(issues::table)
        .execute(&connection)
        .unwrap();

    Redirect::to("/helpdesk")
}