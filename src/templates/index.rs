use crate::model::*;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
        pub assets: &'a Vec<Pantry>,
}
