#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::views::trains::*;
use axum::extract::Path;
use axum::response::IntoResponse;

async fn trains_index() -> impl IntoResponse {
    super::HtmlTemplate(TrainsIndexResponse {
        stations: crate::services::marta::arrivals_by_station().await,
    })
}

async fn trains_station(Path(station_name): Path<String>) -> impl IntoResponse {
    super::HtmlTemplate(TrainsStationResponse {
        arrivals: crate::services::marta::single_station_arrivals(&station_name).await,
        station_name,
    })
}

async fn trains_show(Path(train_id): Path<String>) -> impl IntoResponse {
    super::HtmlTemplate(TrainsShowResponse {
        arrivals: crate::services::marta::single_train_arrivals(&train_id.clone()).await,
        train_id,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(trains_index))
        .add("/stations/:station_name", get(trains_station))
        .add("/trains/:train_id", get(trains_show))
}
