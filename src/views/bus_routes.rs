use askama::Template;
use crate::models::_entities::{shapes, trips};
use crate::controllers::bus_routes::RealtimeRoute;

#[derive(Template)]
#[template(path = "bus_routes/index.html.askama", escape = "html")]
pub struct BusRoutesIndexResponse {
    pub routes: Vec<RealtimeRoute>
}
