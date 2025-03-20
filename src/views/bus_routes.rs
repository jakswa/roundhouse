use crate::controllers::bus_routes::{Index, Show};
use askama::Template;

#[derive(Template)]
#[template(path = "bus_routes/index.html.askama", escape = "html")]
pub struct BusRoutesIndexResponse {
    pub routes: Vec<Index>,
}

#[derive(Template)]
#[template(path = "bus_routes/show.html.askama", escape = "html")]
pub struct BusRoutesShowResponse {
    pub route: Show,
}
