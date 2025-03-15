use crate::controllers::bus_routes::RealtimeRoute;
use askama::Template;

#[derive(Template)]
#[template(path = "bus_routes/index.html.askama", escape = "html")]
pub struct BusRoutesIndexResponse {
    pub routes: Vec<RealtimeRoute>,
}
