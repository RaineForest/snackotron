use crate::model::Asset;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
        pub assets: &'a Vec<Asset>,
}
