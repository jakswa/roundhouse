use crate::controllers::gtfs::VehPos;
use crate::models::_entities::stops::Model as stops_model;
use crate::transit_realtime::TripUpdate;
use askama::Template;

#[derive(Template)]
#[template(path = "stops/index.html.askama", escape = "html")]
pub struct StopsIndexResponse {
    pub stops: Vec<stops_model>,
    pub trip_updates: Vec<TripUpdate>,
    pub vehicles: Vec<VehPos>,
}
