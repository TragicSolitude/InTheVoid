#[derive(FromForm)]
pub struct NewIssueForm {
    summary: String,
    description: String,
    priority: String
}