use askama::Template;

#[derive(Template)]
#[template(path = "trains/index.html.askama", escape = "html")]
pub struct TrainsIndexResponse {
    pub stations: Vec<crate::services::marta::Station>,
}

#[derive(Template)]
#[template(path = "trains/station.html.askama", escape = "html")]
pub struct TrainsStationResponse {
    pub station_name: String,
    pub arrivals: Vec<crate::services::marta::TrainArrival>,
}
