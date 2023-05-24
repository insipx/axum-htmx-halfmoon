//! Askama Template Pages
use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomepageTemplate<'a> {
    pub title: &'a str,
    pub header: &'a str,
}

#[derive(Template)]
#[template(path = "pages/404.html")]
pub struct FourOhFourTemplate<'a> {
    pub title: &'a str,
}
