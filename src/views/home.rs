use askama::Template;

#[derive(Template)]
#[template(path = "home.html.askama", escape = "html")]
pub struct HomeResponse {
    pub app_name: String,
}
