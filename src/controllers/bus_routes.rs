use crate::models::_entities::{routes};
use crate::transit_realtime::{FeedMessage, TripUpdate, VehiclePosition};
use crate::controllers::gtfs::VehPos;
use crate::views::bus_routes::BusRoutesIndexResponse;

use loco_rs::prelude::*;

pub struct RealtimeRoute {
    route: routes::Model,
    trips: Vec<TripUpdate>,
    positions: Vec<VehPos>
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("bus/routes")
        .add("/", get(index))
}

async fn index(state_ctx: State<AppContext>) -> impl IntoResponse {
    let routes = super::gtfs::vehicle_positions(state_ctx).await;

    super::HtmlTemplate(BusRoutesIndexResponse {
        routes: vec![]
    })
}
