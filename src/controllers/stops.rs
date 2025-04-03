use crate::controllers::gtfs::VehPos;
use crate::models::_entities::stops;
use crate::transit_realtime::TripUpdate;
use crate::views::stops::*;
use axum::{extract::Query, Extension};
use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new().prefix("stops").add("/", get(index))
}

#[derive(serde::Deserialize)]
struct StopsQuery {
    name: String,
}
async fn index(
    Query(query): Query<StopsQuery>,
    State(ctx): State<AppContext>,
    Extension(client): Extension<reqwest::Client>,
) -> Result<impl IntoResponse> {
    let like_name = query.name.replace(" ", "%");
    let stops = stops::Entity::find()
        .filter(stops::Column::StopName.like(format!("%{}%", like_name)))
        .all(&ctx.db)
        .await?;

    let default_stop_id = String::from("0");
    let trip_updates: Vec<TripUpdate> = super::gtfs::get_trip_updates(&client)
        .await
        .entity
        .into_iter()
        .filter_map(|i| i.trip_update)
        .filter(|tu| {
            tu.stop_time_update
                .iter()
                .find(|stu| {
                    let stop_id = stu
                        .stop_id
                        .as_ref()
                        .unwrap_or(&default_stop_id)
                        .parse::<i64>()
                        .unwrap();
                    stops.iter().find(|stop| stop.stop_id == stop_id).is_some()
                })
                .is_some()
        })
        .collect();
    let ids = trip_updates
        .iter()
        .filter_map(|tu| tu.trip.trip_id.clone())
        .collect::<Vec<String>>();

    let vehicles: Vec<VehPos> = super::gtfs::get_vehicle_positions(&client)
        .await
        .into_iter()
        .filter(|veh_pos| {
            if let Some(trip_id) = &veh_pos.trip_id {
                return ids.contains(trip_id);
            };
            false
        })
        .collect();

    Ok(super::HtmlTemplate(StopsIndexResponse {
        stops,
        trip_updates,
        vehicles,
    }))
}
