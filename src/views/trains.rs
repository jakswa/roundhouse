use askama::Template;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "trains/index.html.askama", escape = "html")]
pub struct TrainsIndexResponse {
    pub stations: Vec<crate::services::marta::Station>,
    pub starred_stations: Vec<crate::services::marta::Station>,
    pub nearby_stations: Vec<crate::services::marta::Station>,
    pub nearby_enabled: bool,
}

#[derive(Template)]
#[template(path = "trains/station.html.askama", escape = "html")]
pub struct TrainsStationResponse {
    pub station_with_arrivals: crate::services::marta::Station,
    pub is_starred: bool,
    pub train_id: String,
}

#[derive(Template)]
#[template(path = "trains/show.html.askama", escape = "html")]
pub struct TrainsShowResponse {
    pub train_id: String,
    pub arrivals: Vec<Arc<crate::services::marta::TrainArrival>>,
}
