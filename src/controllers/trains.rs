#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::views::trains::TrainsIndexResponse;
use axum::response::IntoResponse;

async fn trains_index() -> impl IntoResponse {
    super::HtmlTemplate(TrainsIndexResponse {
        stations: crate::services::marta::arrivals_by_station().await,
    })
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(trains_index))
}
