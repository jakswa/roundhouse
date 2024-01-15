use askama::Template;

#[derive(Template)]
#[template(path = "home.html.askama", escape = "html")]
pub struct HomeResponse {
    pub stations: Vec<crate::services::marta::Station>,
}
