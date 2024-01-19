use askama::Template;

#[derive(Template)]
#[template(path = "trains/index.html.askama", escape = "html")]
pub struct TrainsIndexResponse {
    pub stations: Vec<crate::services::marta::Station>,
}
