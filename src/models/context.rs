use models::navigation::NavigationItem;

// Macro for generating template context structs that include
// the fields used in the layout partial
// TODO Figure out some way to precompile the nav list into the
// ... templates so that we don't need to include "get_nav_list()"
// ... on *every* instance of one of these structs
macro_rules! context {
    ( $a:ident; { $( $b:ident: $c:ident ),* } ) => {
        #[derive(Serialize)]
        pub struct $a {
            pub nav_links: Vec<NavigationItem>,
            pub current_page: String,
            $(pub $b: $c,)*
        }
    }
}

context!{HelpDeskIndex; {}}

context!{HelpDeskNew; {
    layout: String
}}