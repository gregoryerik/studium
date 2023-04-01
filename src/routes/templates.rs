use askama::Template;

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path="setup.html")]
pub struct SetupTemplate{}


#[derive(Template)]
#[template(path="subject_frame.html")]
pub struct SubjectFrameTemplate<'a> {
    pub title: Option<&'a str>,
    pub subtitle: Option<&'a str>,
    pub deck_size: Option<i32>,
    pub to_update: Option<i32>,
}