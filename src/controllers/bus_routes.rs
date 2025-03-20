use crate::controllers::gtfs::VehPos;
use crate::models::_entities::{routes, stop_times, trips};
use crate::transit_realtime::TripUpdate;
use crate::views::bus_routes::*;

use axum::Extension;
use chrono::TimeZone;
use loco_rs::prelude::*;

pub struct TripSummary {
    pub trip_update: Option<TripUpdate>,
    pub trip: trips::Model,
}

impl TripSummary {
    pub fn stops_left(&self) -> usize {
        self.trip_update
            .as_ref()
            .map(|i| i.stop_time_update.len())
            .unwrap_or(0)
    }
    pub fn start_time(&self) -> &str {
        match self
            .trip_update
            .as_ref()
            .and_then(|i| i.trip.start_time.as_ref())
        {
            Some(val) => val,
            None => "N/A",
        }
    }
}

pub struct Index {
    pub route: routes::Model,
    pub trips: Vec<TripSummary>,
}

pub struct TripDetail {
    pub trip: trips::Model,
    pub next_stop_time: stop_times::Model,
    pub trip_update: TripUpdate,
}

impl TripDetail {
    pub fn stops_left(&self) -> usize {
        self.trip_update.stop_time_update.len()
    }
    pub fn delay(&self) -> i64 {
        let scheduled = chrono::NaiveTime::parse_from_str(
            &self.next_stop_time.departure_time.as_ref().unwrap(),
            "%H:%M:%S",
        )
        .ok()
        .and_then(|i| {
            let date = chrono::Local::now().date_naive();
            chrono::Local
                .from_local_datetime(&date.and_time(i))
                .single()
        })
        .expect("expecting parse-able time");
        let seen = self
            .trip_update
            .stop_time_update
            .first()
            .and_then(|stu| stu.departure.and_then(|dep| dep.time))
            .and_then(|ts| chrono::Utc.timestamp_opt(ts, 0).single());
        if seen.is_none() {
            return 0;
        }
        seen.unwrap().timestamp() - scheduled.timestamp()
    }
    pub fn timeliness(&self) -> String {
        let delay = self.delay();
        let min = (delay / 60).abs();
        // UGH: look at schedule vs API? is "delay" always null/empty bleh
        match delay {
            0 => "on time".to_string(),
            1.. => format!("{min}min late"),
            ..0 => format!("{min}min early"),
        }
    }
    pub fn timeliness_color(&self) -> &str {
        match self.delay() {
            0 => "green",
            1.. => "red",
            ..0 => "pink",
        }
    }
}

pub struct Show {
    pub route: routes::Model,
    pub trips: Vec<TripDetail>,
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("bus/routes")
        .add("/", get(index))
        .add("/{route_id}", get(show))
}

async fn show(
    Path(route_id): Path<i64>,
    State(ctx): State<AppContext>,
    Extension(client): Extension<reqwest::Client>,
) -> Result<impl IntoResponse> {
    let route: routes::Model = routes::Entity::find_by_id(route_id)
        .one(&ctx.db)
        .await?
        .expect("TODO: render standard 404 page");
    let trip_updates: Vec<TripUpdate> = super::gtfs::get_trip_updates(&client)
        .await
        .entity
        .into_iter()
        .filter(|i| {
            i.trip_update
                .as_ref()
                .and_then(|tu| tu.trip.route_id.as_ref())
                .and_then(|i| i.parse::<i64>().ok())
                .unwrap_or(0)
                == route_id
        })
        .filter_map(|i| i.trip_update)
        .collect();
    let trip_ids: Vec<i64> = trip_updates
        .iter()
        .filter_map(|tu| tu.trip.trip_id.as_ref().and_then(|i| i.parse::<i64>().ok()))
        .collect();
    let next_stop_conds = trip_updates
        .iter()
        .map(|tu| {
            let trip_id = tu
                .trip
                .trip_id
                .as_ref()
                .and_then(|i| i.parse::<i64>().ok())
                .expect("trip ID expected");
            let stop_seq = tu
                .stop_time_update
                .first()
                .and_then(|stu| stu.stop_sequence)
                .expect("stop seq expected");
            stop_times::Column::TripId
                .eq(trip_id)
                .and(stop_times::Column::StopSequence.eq(stop_seq))
        })
        .reduce(|i, j| i.or(j))
        .expect("handle None here, empty trip_updates list case");

    let next_stop_times = stop_times::Entity::find()
        .filter(next_stop_conds)
        .all(&ctx.db)
        .await?;
    let trip_models = trips::Entity::find()
        .filter(trips::Column::TripId.is_in(trip_ids))
        .all(&ctx.db)
        .await?;
    let trips: Vec<TripDetail> = trip_updates
        .into_iter()
        .map(|trip_update| {
            let trip = trip_models
                .iter()
                .find(|tm| {
                    tm.trip_id
                        == trip_update
                            .trip
                            .trip_id
                            .as_ref()
                            .and_then(|ti| ti.parse::<i64>().ok())
                            .unwrap_or(0)
                })
                .expect("trip for update")
                .clone();
            let next_stop_time = next_stop_times
                .iter()
                .find(|st| st.trip_id == trip.trip_id)
                .expect("always next stop time?")
                .clone();
            TripDetail {
                trip_update,
                trip,
                next_stop_time,
            }
        })
        .collect();
    let rt_route = Show { route, trips };
    Ok(super::HtmlTemplate(BusRoutesShowResponse {
        route: rt_route,
    }))
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

    let trip_ids: Vec<i64> = all_positions
        .iter()
        .filter_map(|i| i.trip_id.as_ref().and_then(|j| j.parse::<i64>().ok()))
        .collect();
    let seen_trips: Vec<trips::Model> = trips::Entity::find()
        .filter(trips::Column::TripId.is_in(trip_ids))
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
                    let trip_model = seen_trips
                        .iter()
                        .find(|t| {
                            t.trip_id == veh_pos.trip_id.clone().unwrap().parse::<i64>().unwrap()
                        })
                        .expect("expecting trip");
                    TripSummary {
                        trip_update: trip_update.cloned(),
                        trip: trip_model.clone(),
                    }
                })
                .collect();

            Index { route, trips }
        })
        .collect();
    Ok(super::HtmlTemplate(BusRoutesIndexResponse { routes }))
}
