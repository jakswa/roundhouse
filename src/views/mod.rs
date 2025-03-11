use askama::Template;

pub mod trains;
pub mod bus_routes;

#[derive(Template)]
#[template(path = "404.html.askama", escape = "html")]
pub struct Http404Template {
    variation: usize,
}

impl Default for Http404Template {
    fn default() -> Self {
        Self {
            variation: fastrand::usize(..5),
        }
    }
}
