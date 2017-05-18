#[derive(Serialize)]
pub struct NavigationItem {
    pub value: &'static str,
    pub target: &'static str
}

pub fn get_nav_list() -> Vec<NavigationItem> {
    let list = vec![
        NavigationItem {value: "Home", target: "/helpdesk"}
    ];

    list
}