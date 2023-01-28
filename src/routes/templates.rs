use askama::Template;

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path="setup.html")]
pub struct SetupTemplate{}