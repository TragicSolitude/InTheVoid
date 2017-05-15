use models::navigation::NavigationItem;

#[derive(Serialize)]
pub struct HelpDeskIndex {
    pub nav_links: Vec<NavigationItem>,
    pub current_page: String
}