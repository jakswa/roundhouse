use askama::Template;

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
    pub station_name: String,
    pub arrivals: Vec<crate::services::marta::TrainArrival>,
    pub is_starred: bool,
}

#[derive(Template)]
#[template(path = "trains/show.html.askama", escape = "html")]
pub struct TrainsShowResponse {
    pub train_id: String,
    pub arrivals: Vec<crate::services::marta::TrainArrival>,
}
