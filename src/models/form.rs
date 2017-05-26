#[derive(FromForm)]
pub struct NewIssueForm {
    pub summary: String,
    pub description: String,
    pub priority: String
}