use crate::transit_realtime::{FeedMessage, VehiclePosition};
use cached::proc_macro::cached;
use prost::Message; // needed for .decode >_<

use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

use crate::models::_entities::{shapes, trips};

pub fn routes() -> Routes {
    Routes::new()
        .prefix("gtfs/")
        .add("/", get(index))
        .add("trip_updates", get(trip_updates))
        .add("trip_shape/{trip_id}", get(trip_shape))
        .add("vehicle_positions", get(vehicle_positions))
}

#[derive(serde::Serialize, Clone)]
pub struct VehPos {
    pub lat: f32,
    pub lon: f32,
    pub bearing: Option<f32>,
    pub speed: Option<f32>,
    pub trip_id: Option<String>,
    pub route_id: Option<String>,
    pub timestamp: Option<u64>,
    pub label: Option<String>,
    pub vehicle_id: Option<String>,
}

impl From<VehiclePosition> for VehPos {
    fn from(tvp: VehiclePosition) -> Self {
        let pos = tvp.position.unwrap();
        let trip = tvp.trip.unwrap();
        let vehicle = tvp.vehicle.unwrap();
        VehPos {
            lat: pos.latitude,
            lon: pos.longitude,
            bearing: pos.bearing,
            speed: pos.speed,
            trip_id: trip.trip_id,
            route_id: trip.route_id,
            timestamp: tvp.timestamp,
            vehicle_id: vehicle.id,
            label: vehicle.label,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ShapePoint {
    seq: i32,
    lat: f64,
    lon: f64,
}

impl From<shapes::Model> for ShapePoint {
    fn from(shape: shapes::Model) -> Self {
        ShapePoint {
            seq: shape.shape_pt_sequence,
            lat: shape.shape_pt_lat,
            lon: shape.shape_pt_lon,
        }
    }
}

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::json(vec![
        "hi.",
        "this is jake.",
        "try /vehicle_positions or /trip_updates.",
    ])
}

#[debug_handler]
pub async fn trip_updates(
    State(_ctx): State<AppContext>,
    Extension(http): Extension<reqwest::Client>,
) -> Result<Response> {
    format::json(get_trip_updates(&http).await)
}

#[debug_handler]
pub async fn vehicle_positions(
    Extension(http): Extension<reqwest::Client>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::json(get_vehicle_positions(&http).await)
}

#[debug_handler]
pub async fn trip_shape(
    Path(trip_id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let trip = trips::Entity::find_by_id(trip_id)
        .one(&ctx.db)
        .await?
        .expect("trip not found");

    format::json(
        shapes::Entity::find()
            .filter(shapes::Column::ShapeId.eq(trip.shape_id))
            .all(&ctx.db)
            .await?
            .into_iter()
            .map(|model| ShapePoint::from(model))
            .collect::<Vec<ShapePoint>>(),
    )
}

static VEHICLE_POSITIONS: &str =
    "https://gtfs-rt.itsmarta.com/TMGTFSRealTimeWebService/vehicle/vehiclepositions.pb";
static TRIP_UPDATES: &str =
    "https://gtfs-rt.itsmarta.com/TMGTFSRealTimeWebService/tripupdate/tripupdates.pb";

pub async fn get_vehicle_positions(http: &reqwest::Client) -> Vec<VehPos> {
    cached_http_get(VEHICLE_POSITIONS, http)
        .await
        .entity
        .into_iter()
        .filter_map(|i| i.vehicle)
        .map(|i| VehPos::from(i))
        .collect()
}

pub async fn get_trip_updates(http: &reqwest::Client) -> FeedMessage {
    if let Ok(stubfile) = std::env::var("TRIUPD_STUB") {
        let file = std::fs::read(stubfile).expect("TRIUPD stub valid?");
        return FeedMessage::decode(&file[..]).unwrap();
    }
    cached_http_get(TRIP_UPDATES, http).await
}

#[cached(
    time = 10,
    sync_writes = "default",
    key = "String",
    convert = r#"{ String::from(endpoint) }"#
)]
async fn cached_http_get(endpoint: &str, client: &reqwest::Client) -> FeedMessage {
    let resp = client
        .get(endpoint)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    FeedMessage::decode(&resp[..]).unwrap()
}
