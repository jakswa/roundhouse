use crate::controllers::gtfs::VehPos;
use crate::models::_entities::routes;
use crate::transit_realtime::{FeedMessage, TripUpdate, VehiclePosition};
use crate::views::bus_routes::BusRoutesIndexResponse;

use axum::Extension;
use loco_rs::prelude::*;

pub struct RealtimeRoute {
    pub route: routes::Model,
    pub trips: Vec<(Option<TripUpdate>, VehPos)>,
}

pub fn routes() -> Routes {
    Routes::new().prefix("bus/routes").add("/", get(index))
}

async fn index(
    State(ctx): State<AppContext>,
    Extension(client): Extension<reqwest::Client>,
) -> Result<impl IntoResponse> {
    let mut all_positions = super::gtfs::get_vehicle_positions(&client).await;
    let mut all_trips: Vec<TripUpdate> = super::gtfs::get_trip_updates(&client)
        .await
        .entity
        .into_iter()
        .filter_map(|i| i.trip_update)
        .collect();
    let route_ids: Vec<i64> = all_positions
        .iter()
        .filter_map(|i| i.route_id.as_ref().and_then(|j| j.parse::<i64>().ok()))
        .collect();
    let mut seen_routes: Vec<routes::Model> = routes::Entity::find()
        .filter(routes::Column::RouteId.is_in(route_ids))
        .all(&ctx.db)
        .await?;
    seen_routes.sort_by_cached_key(|r| {
        r.route_short_name
            .clone()
            .unwrap_or("".to_string())
            .parse::<i64>()
            .unwrap_or(0)
    });

    let routes = seen_routes
        .into_iter()
        .map(|route| {
            let mut positions: Vec<VehPos> = vec![];
            let (matching, non_matching): (Vec<_>, Vec<_>) =
                all_positions.drain(..).partition(|pos| {
                    pos.route_id
                        .as_ref()
                        .and_then(|id| id.parse::<i64>().ok())
                        .map_or(false, |id| id == route.route_id)
                });
            positions.extend(matching);
            all_positions = non_matching;

            let (trip_match, trip_nonmatch): (Vec<_>, Vec<_>) =
                all_trips.drain(..).partition(|trip_update| {
                    trip_update
                        .trip
                        .route_id
                        .as_ref()
                        .and_then(|id| id.parse::<i64>().ok())
                        .map_or(false, |id| id == route.route_id)
                });
            all_trips = trip_nonmatch;

            let trips = positions
                .into_iter()
                .map(|veh_pos| {
                    let trip_update = trip_match
                        .iter()
                        .find(|trip_update| veh_pos.trip_id == trip_update.trip.trip_id);
                    (trip_update.cloned(), veh_pos)
                })
                .collect();

            RealtimeRoute { route, trips }
        })
        .collect();
    Ok(super::HtmlTemplate(BusRoutesIndexResponse { routes }))
}
